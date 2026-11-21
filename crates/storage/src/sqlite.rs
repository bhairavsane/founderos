use crate::error::StorageError;
use chrono::Utc;
use founderos_domain::{Asset, AssetStatus, Chunk};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteStorage {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteStorage {
    pub fn open_in_memory() -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory().map_err(|e| StorageError::Database(e.to_string()))?;
        let storage = Self { conn: Arc::new(Mutex::new(conn)) };
        storage.migrate()?;
        Ok(storage)
    }

    pub fn open(path: &str) -> Result<Self, StorageError> {
        let conn = Connection::open(path).map_err(|e| StorageError::Database(e.to_string()))?;
        let storage = Self { conn: Arc::new(Mutex::new(conn)) };
        storage.migrate()?;
        Ok(storage)
    }

    pub fn migrate(&self) -> Result<(), StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS assets (
                id TEXT PRIMARY KEY, file_path TEXT NOT NULL, file_name TEXT NOT NULL,
                mime_type TEXT NOT NULL, file_size INTEGER NOT NULL, sha256 TEXT NOT NULL,
                created_at TEXT NOT NULL, modified_at TEXT NOT NULL, indexed_at TEXT,
                status TEXT NOT NULL, metadata TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS chunks (
                id TEXT PRIMARY KEY, asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
                chunk_index INTEGER NOT NULL, modality TEXT NOT NULL, text_content TEXT,
                page_number INTEGER, timestamp_start REAL, timestamp_end REAL, metadata TEXT NOT NULL
            );
            CREATE VIRTUAL TABLE IF NOT EXISTS chunks_fts USING fts5(
                chunk_id UNINDEXED, asset_id UNINDEXED, content, tokenize = 'porter unicode61'
            );",
        )
        .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }


    pub fn insert_asset(&self, asset: &Asset) -> Result<(), StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;
        let status_str = match asset.status {
            AssetStatus::Pending => "pending",
            AssetStatus::Processing => "processing",
            AssetStatus::Indexed => "indexed",
            AssetStatus::Failed => "failed",
        };
        conn.execute(
            "INSERT OR REPLACE INTO assets (id, file_path, file_name, mime_type, file_size, sha256, created_at, modified_at, indexed_at, status, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                asset.id.to_string(),
                asset.file_path,
                asset.file_name,
                asset.mime_type,
                asset.file_size as i64,
                asset.sha256,
                asset.created_at.to_rfc3339(),
                asset.modified_at.to_rfc3339(),
                asset.indexed_at.map(|t| t.to_rfc3339()),
                status_str,
                asset.metadata.to_string(),
            ],
        ).map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    pub fn insert_chunk(&self, chunk: &Chunk) -> Result<(), StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO chunks (id, asset_id, chunk_index, modality, text_content, page_number, timestamp_start, timestamp_end, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                chunk.id.to_string(),
                chunk.asset_id.to_string(),
                chunk.chunk_index,
                chunk.modality.as_str(),
                chunk.text_content,
                chunk.page_number,
                chunk.timestamp_start,
                chunk.timestamp_end,
                chunk.metadata.to_string(),
            ],
        ).map_err(|e| StorageError::Database(e.to_string()))?;

        if let Some(ref text) = chunk.text_content {
            conn.execute(
                "INSERT INTO chunks_fts (chunk_id, asset_id, content) VALUES (?1, ?2, ?3)",
                params![chunk.id.to_string(), chunk.asset_id.to_string(), text],
            ).map_err(|e| StorageError::Database(e.to_string()))?;
        }
        Ok(())
    }

    pub fn search_fts(&self, query: &str, limit: usize) -> Result<Vec<(Uuid, Uuid, f32, String)>, StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;
        let sanitized = query
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|w| !w.is_empty())
            .map(|w| format!("\"{w}\""))
            .collect::<Vec<_>>()
            .join(" OR ");
        let match_expr = if sanitized.is_empty() { query.to_string() } else { sanitized };

        let mut stmt = conn
            .prepare(
                "SELECT chunk_id, asset_id, bm25(chunks_fts) as rank, content
                 FROM chunks_fts
                 WHERE chunks_fts MATCH ?1
                 ORDER BY rank ASC
                 LIMIT ?2",
            )
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let rows = stmt
            .query_map(params![match_expr, limit as i64], |row| {
                let chunk_id_str: String = row.get(0)?;
                let asset_id_str: String = row.get(1)?;
                let rank: f64 = row.get(2)?;
                let snippet: String = row.get(3)?;
                Ok((
                    Uuid::parse_str(&chunk_id_str).unwrap_or_default(),
                    Uuid::parse_str(&asset_id_str).unwrap_or_default(),
                    rank as f32,
                    snippet,
                ))
            })
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r.map_err(|e| StorageError::Database(e.to_string()))?);
        }
        Ok(results)
    }

    pub fn get_asset(&self, id: &Uuid) -> Result<Option<Asset>, StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, file_path, file_name, mime_type, file_size, sha256, created_at, modified_at, indexed_at, status, metadata FROM assets WHERE id = ?1")
            .map_err(|e| StorageError::Database(e.to_string()))?;
        let mut rows = stmt.query(params![id.to_string()]).map_err(|e| StorageError::Database(e.to_string()))?;
        if let Some(row) = rows.next().map_err(|e| StorageError::Database(e.to_string()))? {
            let id_str: String = row.get(0).map_err(|e| StorageError::Database(e.to_string()))?;
            let file_path: String = row.get(1).map_err(|e| StorageError::Database(e.to_string()))?;
            let file_name: String = row.get(2).map_err(|e| StorageError::Database(e.to_string()))?;
            let mime_type: String = row.get(3).map_err(|e| StorageError::Database(e.to_string()))?;
            let file_size: i64 = row.get(4).map_err(|e| StorageError::Database(e.to_string()))?;
            let sha256: String = row.get(5).map_err(|e| StorageError::Database(e.to_string()))?;
            let status_str: String = row.get(9).map_err(|e| StorageError::Database(e.to_string()))?;
            let meta_str: String = row.get(10).map_err(|e| StorageError::Database(e.to_string()))?;
            let status = match status_str.as_str() {
                "indexed" => AssetStatus::Indexed,
                "processing" => AssetStatus::Processing,
                "failed" => AssetStatus::Failed,
                _ => AssetStatus::Pending,
            };
            Ok(Some(Asset {
                id: Uuid::parse_str(&id_str).unwrap_or_default(),
                file_path,
                file_name,
                mime_type,
                file_size: file_size as u64,
                sha256,
                created_at: Utc::now(),
                modified_at: Utc::now(),
                indexed_at: None,
                status,
                metadata: serde_json::from_str(&meta_str).unwrap_or(serde_json::json!({})),
            }))
        } else {
            Ok(None)
        }
    }
}


