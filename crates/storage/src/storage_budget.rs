use crate::error::StorageError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const DEFAULT_STORAGE_BUDGET_BYTES: u64 = 10 * 1024 * 1024 * 1024; // 10 GB

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUsage {
    pub total_used_bytes: u64,
    pub budget_bytes: u64,
    pub used_percentage: f64,
    pub remaining_bytes: u64,
}

pub struct StorageBudgetManager {
    storage_dir: PathBuf,
    budget_bytes: u64,
}

impl StorageBudgetManager {
    pub fn new(storage_dir: PathBuf, budget_bytes: u64) -> Self {
        Self {
            storage_dir,
            budget_bytes: if budget_bytes == 0 {
                DEFAULT_STORAGE_BUDGET_BYTES
            } else {
                budget_bytes
            },
        }
    }

    pub fn default_for_dir(storage_dir: PathBuf) -> Self {
        Self::new(storage_dir, DEFAULT_STORAGE_BUDGET_BYTES)
    }

    pub fn budget_bytes(&self) -> u64 {
        self.budget_bytes
    }

    pub fn storage_dir(&self) -> &Path {
        &self.storage_dir
    }

    pub fn check_usage(&self) -> Result<StorageUsage, StorageError> {
        let total_used = compute_dir_size(&self.storage_dir)?;
        let used_percentage = if self.budget_bytes > 0 {
            (total_used as f64 / self.budget_bytes as f64) * 100.0
        } else {
            0.0
        };
        let remaining_bytes = self.budget_bytes.saturating_sub(total_used);

        Ok(StorageUsage {
            total_used_bytes: total_used,
            budget_bytes: self.budget_bytes,
            used_percentage,
            remaining_bytes,
        })
    }

    pub fn is_over_budget(&self) -> Result<bool, StorageError> {
        let usage = self.check_usage()?;
        Ok(usage.total_used_bytes > usage.budget_bytes)
    }

    pub fn is_warning_level(&self, threshold_ratio: f64) -> Result<bool, StorageError> {
        let usage = self.check_usage()?;
        let ratio = usage.total_used_bytes as f64 / usage.budget_bytes as f64;
        Ok(ratio >= threshold_ratio)
    }
}

fn compute_dir_size(dir: &Path) -> Result<u64, StorageError> {
    if !dir.exists() {
        return Ok(0);
    }
    if dir.is_file() {
        return dir
            .metadata()
            .map(|m| m.len())
            .map_err(|e| StorageError::Database(e.to_string()));
    }

    let mut total = 0u64;
    let entries = std::fs::read_dir(dir).map_err(|e| StorageError::Database(e.to_string()))?;
    for entry in entries {
        let entry = entry.map_err(|e| StorageError::Database(e.to_string()))?;
        let path = entry.path();
        if path.is_dir() {
            total += compute_dir_size(&path)?;
        } else if let Ok(meta) = entry.metadata() {
            total += meta.len();
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_budget_tracking() {
        let temp_dir = std::env::temp_dir().join(format!("founderos_budget_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let file_path = temp_dir.join("dummy.bin");
        std::fs::write(&file_path, vec![0u8; 1024]).unwrap();

        let budget_mgr = StorageBudgetManager::new(temp_dir.clone(), 2048);
        let usage = budget_mgr.check_usage().unwrap();

        assert_eq!(usage.total_used_bytes, 1024);
        assert_eq!(usage.budget_bytes, 2048);
        assert!((usage.used_percentage - 50.0).abs() < 0.1);
        assert_eq!(usage.remaining_bytes, 1024);
        assert!(!budget_mgr.is_over_budget().unwrap());
        assert!(budget_mgr.is_warning_level(0.4).unwrap());
        assert!(!budget_mgr.is_warning_level(0.6).unwrap());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
