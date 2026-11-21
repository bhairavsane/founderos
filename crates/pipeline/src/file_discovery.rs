use crate::error::PipelineError;
use founderos_domain::Modality;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct DiscoveredFile {
    pub path: PathBuf,
    pub name: String,
    pub mime_type: String,
    pub modality: Modality,
    pub size_bytes: u64,
}

pub struct FileScanner {
    pub max_file_size_bytes: u64,
}

impl Default for FileScanner {
    fn default() -> Self {
        Self {
            max_file_size_bytes: 200 * 1024 * 1024, // 200 MB
        }
    }
}

impl FileScanner {
    pub fn scan_directory(&self, root: &Path) -> Result<Vec<DiscoveredFile>, PipelineError> {
        let mut files = Vec::new();
        for entry in WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| !is_ignored(e.path()))
        {
            let entry = entry.map_err(|e| PipelineError::Extraction(e.to_string()))?;
            if entry.file_type().is_file() {
                let path = entry.path().to_path_buf();
                if let Ok(meta) = entry.metadata() {
                    if meta.len() <= self.max_file_size_bytes {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let mime_type = detect_mime_type(&path);
                        let modality = Modality::from_mime_type(&mime_type);
                        files.push(DiscoveredFile {
                            path,
                            name,
                            mime_type,
                            modality,
                            size_bytes: meta.len(),
                        });
                    }
                }
            }
        }
        Ok(files)
    }
}

fn is_ignored(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    if name.starts_with('.') && name != "." {
        return true;
    }
    matches!(name, "node_modules" | "target" | "dist" | "build" | "vendor" | ".git")
}

pub fn detect_mime_type(path: &Path) -> String {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    match ext.as_str() {
        "txt" | "md" | "markdown" => "text/plain".to_string(),
        "pdf" => "application/pdf".to_string(),
        "json" => "application/json".to_string(),
        "csv" => "text/csv".to_string(),
        "rs" | "ts" | "tsx" | "js" | "py" | "go" | "html" => "text/plain".to_string(),
        "png" => "image/png".to_string(),
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "webp" => "image/webp".to_string(),
        "wav" => "audio/wav".to_string(),
        "mp3" => "audio/mp3".to_string(),
        "m4a" => "audio/m4a".to_string(),
        "flac" => "audio/flac".to_string(),
        "mp4" => "video/mp4".to_string(),
        "mov" => "video/quicktime".to_string(),
        "mkv" => "video/x-matroska".to_string(),
        "webm" => "video/webm".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_detection() {
        assert_eq!(detect_mime_type(Path::new("doc.pdf")), "application/pdf");
        assert_eq!(detect_mime_type(Path::new("pic.png")), "image/png");
        assert_eq!(detect_mime_type(Path::new("audio.wav")), "audio/wav");
        assert_eq!(detect_mime_type(Path::new("movie.mp4")), "video/mp4");
    }

    #[test]
    fn test_ignored_rules() {
        assert!(is_ignored(Path::new("node_modules")));
        assert!(is_ignored(Path::new(".git")));
        assert!(!is_ignored(Path::new("src")));
    }
}
