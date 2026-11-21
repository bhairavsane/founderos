use founderos_domain::Chunk;
use uuid::Uuid;

pub struct TextChunker {
    pub max_chars_per_chunk: usize,
    pub overlap_chars: usize,
}

impl Default for TextChunker {
    fn default() -> Self {
        Self {
            max_chars_per_chunk: 800,
            overlap_chars: 100,
        }
    }
}

impl TextChunker {
    pub fn chunk_text(&self, asset_id: Uuid, text: &str, page_number: Option<u32>) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        if len == 0 {
            return chunks;
        }

        let mut start = 0;
        let mut chunk_idx = 0;

        while start < len {
            let mut end = (start + self.max_chars_per_chunk).min(len);
            if end < len {
                if let Some(pos) = chars[start..end].iter().rposition(|&c| c == '\n' || c == '.') {
                    if pos > self.max_chars_per_chunk / 2 {
                        end = start + pos + 1;
                    }
                }
            }
            let chunk_str: String = chars[start..end].iter().collect();
            chunks.push(Chunk::new_text(asset_id, chunk_idx, chunk_str.trim().to_string(), page_number));
            chunk_idx += 1;
            if end >= len {
                break;
            }
            start = end.saturating_sub(self.overlap_chars);
        }
        chunks
    }
}

pub struct AudioChunker {
    pub window_secs: f64,
    pub overlap_secs: f64,
}

impl Default for AudioChunker {
    fn default() -> Self {
        Self {
            window_secs: 30.0,
            overlap_secs: 5.0,
        }
    }
}

impl AudioChunker {
    pub fn chunk_audio(&self, asset_id: Uuid, total_duration_secs: f64, transcript_segments: &[(f64, f64, String)]) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut start = 0.0;
        let mut idx = 0;

        while start < total_duration_secs {
            let end = (start + self.window_secs).min(total_duration_secs);
            let matching_text: Vec<&str> = transcript_segments
                .iter()
                .filter(|(s, e, _)| *s < end && *e > start)
                .map(|(_, _, txt)| txt.as_str())
                .collect();

            let transcript = if matching_text.is_empty() {
                None
            } else {
                Some(matching_text.join(" "))
            };

            chunks.push(Chunk::new_audio_segment(asset_id, idx, start, end, transcript));
            idx += 1;
            if end >= total_duration_secs {
                break;
            }
            start += self.window_secs - self.overlap_secs;
        }
        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_chunking() {
        let chunker = TextChunker { max_chars_per_chunk: 20, overlap_chars: 5 };
        let aid = Uuid::new_v4();
        let chunks = chunker.chunk_text(aid, "Hello world. This is FounderOS test.", None);
        assert!(chunks.len() >= 2);
    }

    #[test]
    fn test_audio_chunking() {
        let chunker = AudioChunker { window_secs: 30.0, overlap_secs: 5.0 };
        let aid = Uuid::new_v4();
        let segs = vec![(10.0, 15.0, "Pricing is $43,820".to_string())];
        let chunks = chunker.chunk_audio(aid, 60.0, &segs);
        assert_eq!(chunks.len(), 3);
        assert!(chunks[0].text_content.as_ref().unwrap().contains("$43,820"));
    }
}
