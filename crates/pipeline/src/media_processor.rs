use crate::error::PipelineError;
use std::path::Path;
use std::process::Command;

pub struct MediaProcessor {
    ffmpeg_binary: String,
}

impl Default for MediaProcessor {
    fn default() -> Self {
        Self {
            ffmpeg_binary: "ffmpeg".to_string(),
        }
    }
}

impl MediaProcessor {
    pub fn new(ffmpeg_binary: String) -> Self {
        Self { ffmpeg_binary }
    }

    pub fn extract_audio_16k(&self, input_path: &Path, output_wav: &Path) -> Result<(), PipelineError> {
        let output = Command::new(&self.ffmpeg_binary)
            .args([
                "-y",
                "-i",
                input_path.to_str().unwrap_or_default(),
                "-ar",
                "16000",
                "-ac",
                "1",
                "-c:a",
                "pcm_s16le",
                output_wav.to_str().unwrap_or_default(),
            ])
            .output()
            .map_err(|e| PipelineError::Media(format!("Failed to run ffmpeg: {e}")))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(PipelineError::Media(format!("FFmpeg audio extraction failed: {err}")));
        }
        Ok(())
    }

    pub fn extract_scene_keyframes(
        &self,
        input_path: &Path,
        output_pattern: &Path,
        scene_threshold: f64,
    ) -> Result<Vec<f64>, PipelineError> {
        let filter = format!("select='gt(scene,{scene_threshold})',showinfo");
        let output = Command::new(&self.ffmpeg_binary)
            .args([
                "-y",
                "-i",
                input_path.to_str().unwrap_or_default(),
                "-vf",
                &filter,
                "-vsync",
                "vfr",
                output_pattern.to_str().unwrap_or_default(),
            ])
            .output()
            .map_err(|e| PipelineError::Media(format!("Failed to run ffmpeg: {e}")))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(PipelineError::Media(format!("FFmpeg scene extraction failed: {err}")));
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut timestamps = Vec::new();
        for line in stderr.lines() {
            if line.contains("pts_time:") {
                if let Some(pos) = line.find("pts_time:") {
                    let sub = &line[pos + 9..];
                    if let Some(end) = sub.find(' ') {
                        if let Ok(ts) = sub[..end].parse::<f64>() {
                            timestamps.push(ts);
                        }
                    }
                }
            }
        }
        Ok(timestamps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let proc = MediaProcessor::default();
        assert_eq!(proc.ffmpeg_binary, "ffmpeg");
    }
}
