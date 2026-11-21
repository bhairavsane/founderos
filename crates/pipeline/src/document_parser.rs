use crate::error::PipelineError;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ParsedPage {
    pub page_number: usize,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ParsedDocument {
    pub file_name: String,
    pub pages: Vec<ParsedPage>,
}

#[derive(Default)]
pub struct DocumentParser;

impl DocumentParser {
    pub fn parse_file(&self, path: &Path) -> Result<ParsedDocument, PipelineError> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("document")
            .to_string();

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        let pages = match ext.as_str() {
            "docx" => self.parse_docx(path)?,
            "pptx" => self.parse_pptx(path)?,
            "pdf" => self.parse_pdf(path)?,
            _ => {
                let content = std::fs::read_to_string(path)
                    .map_err(|e| PipelineError::Extraction(e.to_string()))?;
                vec![ParsedPage {
                    page_number: 1,
                    text: content,
                }]
            }
        };

        Ok(ParsedDocument {
            file_name: name,
            pages,
        })
    }

    pub fn parse_docx(&self, path: &Path) -> Result<Vec<ParsedPage>, PipelineError> {
        let output = Command::new("unzip")
            .args(["-p", path.to_str().unwrap_or_default(), "word/document.xml"])
            .output()
            .map_err(|e| PipelineError::Extraction(format!("Failed to unzip docx: {e}")))?;

        if !output.status.success() {
            return Err(PipelineError::Extraction("Not a valid DOCX archive".into()));
        }

        let raw_xml = String::from_utf8_lossy(&output.stdout);
        let text = extract_text_from_xml(&raw_xml);

        Ok(vec![ParsedPage {
            page_number: 1,
            text,
        }])
    }

    pub fn parse_pptx(&self, path: &Path) -> Result<Vec<ParsedPage>, PipelineError> {
        let list_output = Command::new("unzip")
            .args(["-l", path.to_str().unwrap_or_default()])
            .output()
            .map_err(|e| PipelineError::Extraction(format!("Failed to read pptx: {e}")))?;

        if !list_output.status.success() {
            return Err(PipelineError::Extraction("Not a valid PPTX archive".into()));
        }

        let list_str = String::from_utf8_lossy(&list_output.stdout);
        let mut slide_files: Vec<String> = list_str
            .lines()
            .filter_map(|l| {
                let parts: Vec<&str> = l.split_whitespace().collect();
                parts
                    .last()
                    .filter(|p| p.starts_with("ppt/slides/slide") && p.ends_with(".xml"))
                    .map(|p| p.to_string())
            })
            .collect();
        slide_files.sort();

        let mut pages = Vec::new();
        for (idx, slide) in slide_files.iter().enumerate() {
            if let Ok(out) = Command::new("unzip")
                .args(["-p", path.to_str().unwrap_or_default(), slide])
                .output()
            {
                if out.status.success() {
                    let xml = String::from_utf8_lossy(&out.stdout);
                    let text = extract_text_from_xml(&xml);
                    pages.push(ParsedPage {
                        page_number: idx + 1,
                        text,
                    });
                }
            }
        }

        if pages.is_empty() {
            pages.push(ParsedPage {
                page_number: 1,
                text: String::new(),
            });
        }

        Ok(pages)
    }

    pub fn parse_pdf(&self, path: &Path) -> Result<Vec<ParsedPage>, PipelineError> {
        let bytes = std::fs::read(path)
            .map_err(|e| PipelineError::Extraction(format!("Failed to read pdf: {e}")))?;

        let content = String::from_utf8_lossy(&bytes);
        let mut pages = Vec::new();
        let page_splits: Vec<&str> = content.split("/Type /Page\n").collect();

        if page_splits.len() > 1 {
            for (idx, slice) in page_splits.iter().skip(1).enumerate() {
                let text = extract_pdf_stream_text(slice);
                pages.push(ParsedPage {
                    page_number: idx + 1,
                    text,
                });
            }
        } else {
            let text = extract_pdf_stream_text(&content);
            pages.push(ParsedPage {
                page_number: 1,
                text,
            });
        }

        Ok(pages)
    }
}

fn extract_text_from_xml(xml: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;

    for ch in xml.chars() {
        if ch == '<' {
            in_tag = true;
        } else if ch == '>' {
            in_tag = false;
        } else if !in_tag {
            text.push(ch);
        }
    }
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn extract_pdf_stream_text(stream: &str) -> String {
    let mut extracted = Vec::new();
    let mut inside_paren = false;
    let mut current = String::new();

    for ch in stream.chars() {
        if ch == '(' && !inside_paren {
            inside_paren = true;
            current.clear();
        } else if ch == ')' && inside_paren {
            inside_paren = false;
            if current.len() > 1 {
                extracted.push(current.clone());
            }
        } else if inside_paren {
            current.push(ch);
        }
    }

    if extracted.is_empty() {
        stream
            .lines()
            .filter(|l| !l.starts_with('/') && !l.starts_with('%') && l.len() > 3)
            .take(20)
            .collect::<Vec<&str>>()
            .join(" ")
    } else {
        extracted.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text_from_xml() {
        let xml = "<w:p><w:r><w:t>Hello FounderOS Team!</w:t></w:r></w:p>";
        let text = extract_text_from_xml(xml);
        assert_eq!(text, "Hello FounderOS Team!");
    }

    #[test]
    fn test_extract_pdf_stream_text() {
        let stream = "BT /F1 12 Tf (Confidential Cap Table) Tj (Total Valuation: $20M) Tj ET";
        let text = extract_pdf_stream_text(stream);
        assert!(text.contains("Confidential Cap Table"));
        assert!(text.contains("Total Valuation: $20M"));
    }
}
