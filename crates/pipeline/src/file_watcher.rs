use crate::error::PipelineError;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum FsChangeEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
}

pub struct DirectoryWatcher {
    _watcher: RecommendedWatcher,
}

impl DirectoryWatcher {
    pub fn new(
        paths_to_watch: &[&Path],
        tx: mpsc::UnboundedSender<FsChangeEvent>,
    ) -> Result<Self, PipelineError> {
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    for path in event.paths {
                        let change = match event.kind {
                            notify::EventKind::Create(_) => Some(FsChangeEvent::Created(path)),
                            notify::EventKind::Modify(_) => Some(FsChangeEvent::Modified(path)),
                            notify::EventKind::Remove(_) => Some(FsChangeEvent::Deleted(path)),
                            _ => None,
                        };
                        if let Some(c) = change {
                            let _ = tx.send(c);
                        }
                    }
                }
            },
            Config::default(),
        )
        .map_err(|e| PipelineError::Extraction(e.to_string()))?;

        for p in paths_to_watch {
            watcher
                .watch(p, RecursiveMode::Recursive)
                .map_err(|e| PipelineError::Extraction(e.to_string()))?;
        }

        Ok(Self { _watcher: watcher })
    }
}
