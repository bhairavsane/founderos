use crate::error::RuntimeError;
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use uuid::Uuid;

pub struct SidecarServer {
    binary_path: PathBuf,
    model_path: PathBuf,
    projector_path: Option<PathBuf>,
    port: u16,
    api_key: String,
    process: Option<Child>,
}

impl SidecarServer {
    pub fn new(binary_path: PathBuf, model_path: PathBuf, projector_path: Option<PathBuf>) -> Result<Self, RuntimeError> {
        let port = pick_ephemeral_port()?;
        let api_key = Uuid::new_v4().to_string();

        Ok(Self {
            binary_path,
            model_path,
            projector_path,
            port,
            api_key,
            process: None,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn base_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }

    pub fn start(&mut self) -> Result<(), RuntimeError> {
        if self.is_running() {
            return Ok(());
        }

        let mut cmd = Command::new(&self.binary_path);
        cmd.args([
            "--port",
            &self.port.to_string(),
            "--host",
            "127.0.0.1",
            "-m",
            self.model_path.to_str().unwrap_or_default(),
            "--embeddings",
            "--api-key",
            &self.api_key,
        ]);

        if let Some(ref proj) = self.projector_path {
            cmd.args(["--mmproj", proj.to_str().unwrap_or_default()]);
        }

        // Stdin piped for EOF watchdog on unix
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let child = cmd
            .spawn()
            .map_err(|e| RuntimeError::Embedding(format!("Failed to spawn llama-server: {e}")))?;

        self.process = Some(child);
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

impl Drop for SidecarServer {
    fn drop(&mut self) {
        self.stop();
    }
}

fn pick_ephemeral_port() -> Result<u16, RuntimeError> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| RuntimeError::Embedding(format!("Failed to bind ephemeral port: {e}")))?;
    let port = listener
        .local_addr()
        .map_err(|e| RuntimeError::Embedding(format!("Failed to get local addr: {e}")))?
        .port();
    Ok(port)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_ephemeral_port_and_security() {
        let server = SidecarServer::new(
            Path::new("dummy-server").to_path_buf(),
            Path::new("model.gguf").to_path_buf(),
            Some(Path::new("mmproj.gguf").to_path_buf()),
        )
        .unwrap();

        assert!(server.port() > 1024);
        assert!(server.base_url().starts_with("http://127.0.0.1:"));
        assert!(server.auth_header().starts_with("Bearer "));
        assert!(!server.is_running());
    }
}
