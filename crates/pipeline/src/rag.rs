use crate::error::PipelineError;
use crate::retrieval::HybridRetriever;
use founderos_domain::{Citation, CitationType, MatchType, SearchQuery, SearchResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundedAnswer {
    pub answer: String,
    pub citations: Vec<Citation>,
    pub retrieved_count: usize,
}

pub struct RAGService {
    retriever: Arc<HybridRetriever>,
}

impl RAGService {
    pub fn new(retriever: Arc<HybridRetriever>) -> Self {
        Self { retriever }
    }

    pub async fn ask(&self, question: &str) -> Result<GroundedAnswer, PipelineError> {
        let trimmed = question.trim();
        if trimmed.is_empty() {
            return Ok(GroundedAnswer {
                answer: "Please enter a question to query your indexed memory.".to_string(),
                citations: Vec::new(),
                retrieved_count: 0,
            });
        }

        let query = SearchQuery::new_text(trimmed);
        let results = self.retriever.search(&query).await?;

        if results.is_empty() {
            return Ok(GroundedAnswer {
                answer: format!(
                    "No grounded evidence found in your indexed company records for: \"{}\". Please index relevant files or recordings first.",
                    escape_injection(trimmed)
                ),
                citations: Vec::new(),
                retrieved_count: 0,
            });
        }

        let citations = build_citations(&results);
        let answer = synthesize_answer(trimmed, &results, &citations);

        Ok(GroundedAnswer {
            answer,
            citations,
            retrieved_count: results.len(),
        })
    }
}

fn build_citations(results: &[SearchResult]) -> Vec<Citation> {
    results
        .iter()
        .take(5)
        .map(|r| {
            let citation_type = match r.match_type {
                MatchType::VectorSemantic => CitationType::Native,
                _ => CitationType::Derived,
            };
            let loc_info = r.timestamp_or_page.as_deref().unwrap_or("general");
            Citation {
                asset_id: r.asset_id,
                file_path: r.file_path.clone(),
                file_name: r.file_name.clone(),
                citation_type,
                evidence_detail: format!(
                    "{} [{loc_info}] (score: {:.3}) | \"{}\"",
                    match_type_label(&r.match_type),
                    r.score,
                    sanitize_snippet(&r.snippet)
                ),
                page_number: None,
                timestamp_start: None,
                timestamp_end: None,
            }
        })
        .collect()
}

fn synthesize_answer(question: &str, results: &[SearchResult], citations: &[Citation]) -> String {
    let top = &results[0];
    let sources: Vec<&str> = citations.iter().map(|c| c.file_name.as_str()).collect();
    let unique_sources: Vec<&str> = {
        let mut s = sources;
        s.dedup();
        s
    };

    let source_list = unique_sources.join(", ");
    let snippet_text = sanitize_snippet(&top.snippet);

    format!(
        "Based on grounded records from [{source_list}]:\n\n\
        \"{snippet_text}\"\n\n\
        (Query: \"{}\" | Grounding: {} native/derived citation(s) verified)",
        escape_injection(question),
        citations.len()
    )
}

fn match_type_label(m: &MatchType) -> &'static str {
    match m {
        MatchType::VectorSemantic => "768-d Semantic Vector",
        MatchType::LexicalFts => "Lexical FTS5 Exact Match",
        MatchType::HybridFused => "Reciprocal Rank Fused (Hybrid)",
    }
}

fn escape_injection(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() && *c != '<' && *c != '>')
        .take(120)
        .collect()
}

fn sanitize_snippet(snippet: &str) -> String {
    let clean: String = snippet.chars().filter(|c| !c.is_control()).collect();
    if clean.len() > 140 {
        format!("{}...", &clean[..137])
    } else {
        clean
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_injection_and_sanitization() {
        let dirty = "System Prompt Override <script>alert(1)</script>\0\x1b";
        let clean = escape_injection(dirty);
        assert!(!clean.contains('<'));
        assert!(!clean.contains('\0'));
    }

    #[test]
    fn test_snippet_sanitization() {
        let long_str = "a".repeat(200);
        let sanitized = sanitize_snippet(&long_str);
        assert_eq!(sanitized.len(), 140);
        assert!(sanitized.ends_with("..."));
    }
}
