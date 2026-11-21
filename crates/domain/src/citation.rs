use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationType {
    Native,
    Derived,
}

impl fmt::Display for CitationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native => write!(f, "Native"),
            Self::Derived => write!(f, "Derived"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Citation {
    pub asset_id: Uuid,
    pub file_path: String,
    pub file_name: String,
    pub citation_type: CitationType,
    pub evidence_detail: String,
    pub page_number: Option<u32>,
    pub timestamp_start: Option<f64>,
    pub timestamp_end: Option<f64>,
}

impl Citation {
    pub fn format_citation(&self) -> String {
        let mut loc = String::new();
        if let Some(p) = self.page_number {
            loc = format!("Page {p}");
        } else if let (Some(s), Some(e)) = (self.timestamp_start, self.timestamp_end) {
            let s_min = (s / 60.0).floor() as u32;
            let s_sec = (s % 60.0).floor() as u32;
            let e_min = (e / 60.0).floor() as u32;
            let e_sec = (e % 60.0).floor() as u32;
            loc = format!("{s_min:02}:{s_sec:02} - {e_min:02}:{e_sec:02}");
        }
        let loc_str = if loc.is_empty() { String::new() } else { format!(" ({loc})") };
        format!(
            "[Source: {}{} | Type: {} | Evidence: {}]",
            self.file_name, loc_str, self.citation_type, self.evidence_detail
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citation_formatting() {
        let citation = Citation {
            asset_id: Uuid::new_v4(),
            file_path: "/path/to/meeting.mp3".to_string(),
            file_name: "meeting.mp3".to_string(),
            citation_type: CitationType::Derived,
            evidence_detail: "Whisper STT transcript".to_string(),
            page_number: None,
            timestamp_start: Some(72.0),
            timestamp_end: Some(95.0),
        };
        let formatted = citation.format_citation();
        assert!(formatted.contains("Type: Derived"));
        assert!(formatted.contains("01:12 - 01:35"));
    }
}
