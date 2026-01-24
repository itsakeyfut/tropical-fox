use bevy::prelude::*;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::time::SystemTime;

use crate::core::ReloadRecord;

/// Handle to the hot-reloaded asset
#[derive(Resource)]
pub struct HotAssetHandle<T: Asset>(pub Handle<T>);

/// Verbose logging configuration
#[derive(Resource)]
pub struct VerboseLogging(pub bool);

/// Reload metrics (optional)
#[derive(Resource)]
pub struct ReloadMetrics<T: Asset> {
    pub reload_count: u64,
    pub success_count: u64,
    pub failure_count: u64,
    pub last_reload_time: Option<SystemTime>,
    pub history: VecDeque<ReloadRecord>,
    _phantom: PhantomData<T>,
}

impl<T: Asset> ReloadMetrics<T> {
    pub fn new(history_size: usize) -> Self {
        Self {
            reload_count: 0,
            success_count: 0,
            failure_count: 0,
            last_reload_time: None,
            history: VecDeque::with_capacity(history_size),
            _phantom: PhantomData,
        }
    }

    pub fn record_success(&mut self) {
        self.reload_count += 1;
        self.success_count += 1;
        self.last_reload_time = Some(SystemTime::now());

        self.history.push_back(ReloadRecord {
            timestamp: SystemTime::now(),
            success: true,
            error_message: None,
        });

        if self.history.len() > 50 {
            self.history.pop_front();
        }
    }

    pub fn record_failure(&mut self, error: String) {
        self.reload_count += 1;
        self.failure_count += 1;
        self.last_reload_time = Some(SystemTime::now());

        self.history.push_back(ReloadRecord {
            timestamp: SystemTime::now(),
            success: false,
            error_message: Some(error),
        });

        if self.history.len() > 50 {
            self.history.pop_front();
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.reload_count == 0 {
            0.0
        } else {
            self.success_count as f64 / self.reload_count as f64
        }
    }
}
