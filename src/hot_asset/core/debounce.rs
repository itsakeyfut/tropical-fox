use bevy::prelude::Resource;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

#[derive(Resource)]
pub struct DebounceTracker {
    last_events: HashMap<PathBuf, SystemTime>,
    duration: Duration,
}

impl DebounceTracker {
    pub fn new(duration: Duration) -> Self {
        Self {
            last_events: HashMap::new(),
            duration,
        }
    }

    pub fn should_reload(&self, path: &Path) -> bool {
        if let Some(last_time) = self.last_events.get(path) {
            if let Ok(elapsed) = SystemTime::now().duration_since(*last_time) {
                return elapsed >= self.duration;
            }
        }
        true
    }

    pub fn mark_reloaded(&mut self, path: PathBuf) {
        self.last_events.insert(path, SystemTime::now());
    }
}
