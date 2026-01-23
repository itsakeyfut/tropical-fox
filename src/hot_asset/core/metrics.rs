use std::collections::VecDeque;
use std::time::SystemTime;

pub struct MetricsTracker {
    pub total_reloads: u64,
    pub successful_reloads: u64,
    pub failed_reloads: u64,
    pub last_reload_time: Option<SystemTime>,
    pub history: VecDeque<ReloadRecord>,
    history_size: usize,
}

#[derive(Debug, Clone)]
pub struct ReloadRecord {
    pub timestamp: SystemTime,
    pub success: bool,
    pub error_message: Option<String>,
}

impl MetricsTracker {
    pub fn new(history_size: usize) -> Self {
        Self {
            total_reloads: 0,
            successful_reloads: 0,
            failed_reloads: 0,
            last_reload_time: None,
            history: VecDeque::with_capacity(history_size),
            history_size,
        }
    }

    pub fn record_success(&mut self) {
        self.total_reloads += 1;
        self.successful_reloads += 1;
        self.last_reload_time = Some(SystemTime::now());

        self.history.push_back(ReloadRecord {
            timestamp: SystemTime::now(),
            success: true,
            error_message: None,
        });

        if self.history.len() > self.history_size {
            self.history.pop_front();
        }
    }

    pub fn record_failure(&mut self, error: String) {
        self.total_reloads += 1;
        self.failed_reloads += 1;
        self.last_reload_time = Some(SystemTime::now());

        self.history.push_back(ReloadRecord {
            timestamp: SystemTime::now(),
            success: false,
            error_message: Some(error),
        });

        if self.history.len() > self.history_size {
            self.history.pop_front();
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_reloads == 0 {
            0.0
        } else {
            self.successful_reloads as f64 / self.total_reloads as f64
        }
    }
}
