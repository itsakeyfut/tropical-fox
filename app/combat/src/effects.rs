//! Combat visual effects
//!
//! Implements hit stop (brief pause on impact) and screen shake effects
//! to provide satisfying combat feedback.

use bevy::prelude::*;
use bevy::time::{Real, Virtual};
use rand::Rng;

use super::{HitStopEvent, ScreenShakeEvent};

/// Resource for tracking hit stop state
#[derive(Resource, Debug)]
pub struct HitStopTimer {
    /// Whether hit stop is currently active
    pub active: bool,
    /// Duration timer (ticks in real time, not virtual)
    pub duration: Timer,
}

impl Default for HitStopTimer {
    fn default() -> Self {
        Self {
            active: false,
            duration: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

/// Resource for tracking screen shake state
#[derive(Resource, Debug)]
pub struct ScreenShake {
    /// Current shake intensity
    pub intensity: f32,
    /// Duration timer
    pub duration: Timer,
    /// Original camera position before shake started
    pub original_pos: Vec3,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            duration: Timer::from_seconds(0.0, TimerMode::Once),
            original_pos: Vec3::ZERO,
        }
    }
}

impl ScreenShake {
    /// Trigger a new screen shake effect
    pub fn trigger(&mut self, intensity: f32, duration_secs: f32) {
        self.intensity = intensity;
        self.duration = Timer::from_seconds(duration_secs, TimerMode::Once);
    }
}

/// Process hit stop events and pause game time
pub fn hitstop_system(
    real_time: Res<Time<Real>>,
    mut time: ResMut<Time<Virtual>>,
    mut hitstop: ResMut<HitStopTimer>,
    mut events: MessageReader<HitStopEvent>,
) {
    // Process incoming events
    for event in events.read() {
        hitstop.active = true;
        hitstop.duration = Timer::from_seconds(event.duration_secs, TimerMode::Once);
        time.set_relative_speed(0.0); // Pause game time
    }

    // Update hit stop timer (using real time, not virtual)
    if hitstop.active {
        // Tick using real time delta (frame-rate independent)
        hitstop.duration.tick(real_time.delta());

        if hitstop.duration.is_finished() {
            hitstop.active = false;
            time.set_relative_speed(1.0); // Resume normal speed
        }
    }
}

/// Process screen shake events and apply camera offset
pub fn screen_shake_system(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut events: MessageReader<ScreenShakeEvent>,
) {
    // Process incoming events
    for event in events.read() {
        shake.trigger(event.intensity, event.duration_secs);

        // Store original camera position
        if let Ok(camera_transform) = camera_query.single() {
            shake.original_pos = camera_transform.translation;
        }
    }

    // Update shake
    if !shake.duration.is_finished() {
        shake.duration.tick(time.delta());

        // Calculate current intensity with decay
        let progress = shake.duration.fraction();
        let current_intensity = shake.intensity * (1.0 - progress);

        // Only apply shake if intensity is meaningful
        if current_intensity > 0.01 {
            let mut rng = rand::rng();
            let offset = Vec3::new(
                rng.random_range(-current_intensity..current_intensity),
                rng.random_range(-current_intensity..current_intensity),
                0.0,
            );

            for mut transform in camera_query.iter_mut() {
                transform.translation = shake.original_pos + offset;
            }
        }
    } else if shake.intensity > 0.0 {
        // Shake finished - restore original position
        for mut transform in camera_query.iter_mut() {
            transform.translation = shake.original_pos;
        }
        shake.intensity = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hitstop_timer_default() {
        let timer = HitStopTimer::default();
        assert!(!timer.active);
    }

    #[test]
    fn test_screen_shake_trigger() {
        let mut shake = ScreenShake::default();
        shake.trigger(10.0, 0.5);
        assert_eq!(shake.intensity, 10.0);
        assert!(!shake.duration.finished());
    }
}
