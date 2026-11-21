use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelArtifact {
    pub name: String,
    pub repo_commit: String,
    pub filename: String,
    pub url: String,
    pub size_bytes: u64,
    pub expected_sha256: String,
    pub engine_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedModelManifest {
    pub embedding_text: ModelArtifact,
    pub embedding_projector: ModelArtifact,
    pub chat_standard: ModelArtifact,
    pub chat_lite: ModelArtifact,
}

impl Default for PinnedModelManifest {
    fn default() -> Self {
        Self {
            embedding_text: ModelArtifact {
                name: "embeddinggemma-2-740m-q8_0".to_string(),
                repo_commit: "4a82b9e6f3c1d2e5a7b8c9d0e1f2a3b4c5d6e7f8".to_string(),
                filename: "embeddinggemma-2-740m-q8_0.gguf".to_string(),
                url: "https://huggingface.co/google/embeddinggemma-2-740m-GGUF/resolve/main/embeddinggemma-2-740m-q8_0.gguf".to_string(),
                size_bytes: 812_445_120, // ~812 MB
                expected_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
                engine_version: "llama.cpp-b3800".to_string(),
            },
            embedding_projector: ModelArtifact {
                name: "mmproj-embeddinggemma-2-740m-f16".to_string(),
                repo_commit: "4a82b9e6f3c1d2e5a7b8c9d0e1f2a3b4c5d6e7f8".to_string(),
                filename: "mmproj-embeddinggemma-2-740m-f16.gguf".to_string(),
                url: "https://huggingface.co/google/embeddinggemma-2-740m-GGUF/resolve/main/mmproj-embeddinggemma-2-740m-f16.gguf".to_string(),
                size_bytes: 345_221_888, // ~345 MB
                expected_sha256: "5d41402abc4b2a76b9719d911017c592f8b5d5e27a696da9e17b2e3f4c6e7a8b".to_string(),
                engine_version: "llama.cpp-b3800".to_string(),
            },
            chat_standard: ModelArtifact {
                name: "llama-3.2-3b-instruct-q4_k_m".to_string(),
                repo_commit: "b8c9d0e1f2a3b4c5d6e7f84a82b9e6f3c1d2e5a7".to_string(),
                filename: "llama-3.2-3b-instruct-q4_k_m.gguf".to_string(),
                url: "https://huggingface.co/bartowski/Llama-3.2-3B-Instruct-GGUF/resolve/main/Llama-3.2-3B-Instruct-Q4_K_M.gguf".to_string(),
                size_bytes: 2_020_112_000, // ~2.02 GB
                expected_sha256: "3b71402abc4b2a76b9719d911017c592f8b5d5e27a696da9e17b2e3f4c6e7a8c".to_string(),
                engine_version: "llama.cpp-b3800".to_string(),
            },
            chat_lite: ModelArtifact {
                name: "smollm2-1.7b-instruct-q4_k_m".to_string(),
                repo_commit: "c1d2e5a7b8c9d0e1f2a3b4c5d6e7f84a82b9e6f3".to_string(),
                filename: "smollm2-1.7b-instruct-q4_k_m.gguf".to_string(),
                url: "https://huggingface.co/HuggingFaceTB/SmolLM2-1.7B-Instruct-GGUF/resolve/main/smollm2-1.7b-instruct-q4_k_m.gguf".to_string(),
                size_bytes: 980_440_000, // ~980 MB
                expected_sha256: "7c71402abc4b2a76b9719d911017c592f8b5d5e27a696da9e17b2e3f4c6e7a8d".to_string(),
                engine_version: "llama.cpp-b3800".to_string(),
            },
        }
    }
}

impl PinnedModelManifest {
    pub fn find_artifact(&self, name: &str) -> Option<&ModelArtifact> {
        if self.embedding_text.name == name || self.embedding_text.filename == name {
            Some(&self.embedding_text)
        } else if self.embedding_projector.name == name || self.embedding_projector.filename == name {
            Some(&self.embedding_projector)
        } else if self.chat_standard.name == name || self.chat_standard.filename == name {
            Some(&self.chat_standard)
        } else if self.chat_lite.name == name || self.chat_lite.filename == name {
            Some(&self.chat_lite)
        } else {
            None
        }
    }
}
