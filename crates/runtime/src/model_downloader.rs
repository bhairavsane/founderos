use crate::error::RuntimeError;
use crate::model_manifest::{ModelArtifact, PinnedModelManifest};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub model_name: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percentage: f64,
    pub speed_mbps: f64,
    pub status: String,
}

pub struct ModelDownloader {
    models_dir: PathBuf,
    manifest: PinnedModelManifest,
}

impl ModelDownloader {
    pub fn new(models_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&models_dir).unwrap_or_default();
        Self {
            models_dir,
            manifest: PinnedModelManifest::default(),
        }
    }

    pub fn models_dir(&self) -> &Path {
        &self.models_dir
    }

    pub fn manifest(&self) -> &PinnedModelManifest {
        &self.manifest
    }

    pub fn is_model_installed(&self, filename: &str) -> bool {
        self.models_dir.join(filename).exists()
    }

    pub fn get_model_path(&self, filename: &str) -> Option<PathBuf> {
        let path = self.models_dir.join(filename);
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    pub fn verify_file_sha256(path: &Path, expected_sha256: &str) -> Result<bool, RuntimeError> {
        let mut file = std::fs::File::open(path)
            .map_err(|e| RuntimeError::Embedding(format!("Failed to open file for checksum: {e}")))?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 64 * 1024];

        loop {
            let bytes_read = file
                .read(&mut buffer)
                .map_err(|e| RuntimeError::Embedding(format!("Read error: {e}")))?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        let result = hasher.finalize();
        let hex_hash = format!("{:x}", result);
        Ok(hex_hash.eq_ignore_ascii_case(expected_sha256))
    }

    pub fn import_offline_model(
        &self,
        source_path: &Path,
        artifact: &ModelArtifact,
    ) -> Result<PathBuf, RuntimeError> {
        if !source_path.exists() {
            return Err(RuntimeError::Embedding(format!("Source path does not exist: {:?}", source_path)));
        }

        let is_valid = Self::verify_file_sha256(source_path, &artifact.expected_sha256)?;
        if !is_valid {
            return Err(RuntimeError::Embedding(format!(
                "Checksum mismatch for offline model {}",
                artifact.name
            )));
        }

        let target_path = self.models_dir.join(&artifact.filename);
        std::fs::copy(source_path, &target_path)
            .map_err(|e| RuntimeError::Embedding(format!("Failed to copy model: {e}")))?;

        Ok(target_path)
    }

    pub async fn simulate_download(
        &self,
        artifact: &ModelArtifact,
        progress_tx: Option<mpsc::UnboundedSender<DownloadProgress>>,
    ) -> Result<PathBuf, RuntimeError> {
        let target_path = self.models_dir.join(&artifact.filename);
        let total = artifact.size_bytes;

        for step in 1..=10 {
            let current = (total / 10) * step;
            let pct = (step as f64) * 10.0;
            if let Some(ref tx) = progress_tx {
                let _ = tx.send(DownloadProgress {
                    model_name: artifact.name.clone(),
                    downloaded_bytes: current,
                    total_bytes: total,
                    percentage: pct,
                    speed_mbps: 45.2,
                    status: if step == 10 { "complete".into() } else { "downloading".into() },
                });
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        }

        // Write marker file for offline tests
        std::fs::write(&target_path, b"FOUNDEROS_GGUF_MOCK_WEIGHTS")
            .map_err(|e| RuntimeError::Embedding(format!("Failed to save weights: {e}")))?;

        Ok(target_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_downloader_manifest_and_simulation() {
        let temp_dir = std::env::temp_dir().join(format!("founderos_models_{}", std::process::id()));
        let downloader = ModelDownloader::new(temp_dir.clone());

        let artifact = downloader.manifest().embedding_text.clone();
        assert_eq!(artifact.filename, "embeddinggemma-2-740m-q8_0.gguf");

        let (tx, mut rx) = mpsc::unbounded_channel();
        let path = downloader.simulate_download(&artifact, Some(tx)).await.unwrap();

        assert!(path.exists());
        assert!(downloader.is_model_installed(&artifact.filename));

        let mut count = 0;
        while let Ok(msg) = rx.try_recv() {
            count += 1;
            assert_eq!(msg.model_name, artifact.name);
        }
        assert_eq!(count, 10);

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
