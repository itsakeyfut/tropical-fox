use crossbeam_channel::{Receiver, Sender, unbounded};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WatchError {
    #[error("Failed to create watcher: {0}")]
    CreateFailed(#[from] notify::Error),

    #[error("Failed to watch path: {0}")]
    WatchFailed(String),
}

#[derive(Debug)]
pub enum FileEvent {
    Modified(PathBuf),
    Created(PathBuf),
}

pub struct FileWatcher {
    _watcher: RecommendedWatcher,
    path: PathBuf,
}

impl FileWatcher {
    pub fn new(path: PathBuf) -> Result<(Self, Receiver<FileEvent>), WatchError> {
        let (tx, rx): (Sender<FileEvent>, Receiver<FileEvent>) = unbounded();

        let watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    // Only process Modify and Create events
                    if matches!(
                        event.kind,
                        notify::EventKind::Modify(_) | notify::EventKind::Create(_)
                    ) {
                        for path in event.paths {
                            // Filter by .ron extension
                            if path.extension().and_then(|s| s.to_str()) == Some("ron") {
                                let _ = tx.send(FileEvent::Modified(path));
                            }
                        }
                    }
                }
            },
            Config::default(),
        )?;

        Ok((
            Self {
                _watcher: watcher,
                path,
            },
            rx,
        ))
    }

    pub fn watch(&mut self) -> Result<(), WatchError> {
        self._watcher
            .watch(&self.path, RecursiveMode::NonRecursive)
            .map_err(|e| WatchError::WatchFailed(e.to_string()))
    }
}
