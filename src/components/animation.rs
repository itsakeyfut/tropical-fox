//! Animation components for sprite-based animation system

use bevy::prelude::*;
use std::collections::HashMap;

/// An animation clip defines a range of frames and playback speed
#[derive(Clone, Debug)]
pub struct AnimationClip {
    /// Index of the first frame in the texture atlas
    pub first_frame: usize,
    /// Index of the last frame in the texture atlas
    pub last_frame: usize,
    /// Frames per second (playback speed)
    pub fps: f32,
}

impl AnimationClip {
    /// Create a new animation clip
    pub fn new(first_frame: usize, last_frame: usize, fps: f32) -> Self {
        Self {
            first_frame,
            last_frame,
            fps,
        }
    }

    /// Get the total number of frames in this clip
    pub fn frame_count(&self) -> usize {
        self.last_frame - self.first_frame + 1
    }
}

/// Animation controller manages available animations and tracks current animation
#[derive(Component, Debug)]
pub struct AnimationController {
    /// Map of animation name to animation clip
    pub animations: HashMap<String, AnimationClip>,
    /// Name of the currently playing animation
    pub current_animation: String,
    /// Name of the previously played animation (for transition logic)
    pub previous_animation: String,
}

impl AnimationController {
    /// Create a new animation controller with empty animations
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current_animation: String::new(),
            previous_animation: String::new(),
        }
    }

    /// Add an animation clip to the controller
    pub fn add_animation(&mut self, name: impl Into<String>, clip: AnimationClip) {
        self.animations.insert(name.into(), clip);
    }

    /// Play an animation by name, restarting if it's a different animation
    pub fn play(&mut self, animation: &str, state: &mut AnimationState) {
        if let Some(clip) = self.animations.get(animation) {
            // Only restart if switching to a different animation
            if self.current_animation != animation {
                self.previous_animation = self.current_animation.clone();
                self.current_animation = animation.to_string();
                state.current_frame = clip.first_frame;

                // Update timer duration based on the clip's fps
                let frame_duration = 1.0 / clip.fps;
                state
                    .timer
                    .set_duration(std::time::Duration::from_secs_f32(frame_duration));
                state.timer.reset();
                state.playing = true;
            }
        } else {
            warn!("Attempted to play non-existent animation: {}", animation);
        }
    }

    /// Get the current animation clip, if one exists
    pub fn current_clip(&self) -> Option<&AnimationClip> {
        self.animations.get(&self.current_animation)
    }
}

impl Default for AnimationController {
    fn default() -> Self {
        Self::new()
    }
}

/// Animation state tracks the current playback state of an animation
#[derive(Component, Debug)]
pub struct AnimationState {
    /// Current frame index being displayed
    pub current_frame: usize,
    /// Timer for frame progression
    pub timer: Timer,
    /// Whether the animation is currently playing
    pub playing: bool,
    /// Whether the animation should loop when it reaches the end
    pub looping: bool,
}

impl AnimationState {
    /// Create a new animation state
    pub fn new(fps: f32, looping: bool) -> Self {
        Self {
            current_frame: 0,
            timer: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
            playing: false,
            looping,
        }
    }

    /// Reset the animation state to the beginning
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.timer.reset();
        self.playing = true;
    }
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::new(10.0, true) // Default to 10 fps and looping
    }
}

/// Animation events that can be triggered at specific frames
#[derive(Component, Debug, Default)]
pub struct AnimationEvents {
    /// Map of frame index to events that should trigger on that frame
    pub events: HashMap<usize, Vec<AnimationEvent>>,
}

impl AnimationEvents {
    /// Create new empty animation events
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an event to trigger at a specific frame
    pub fn add_event(&mut self, frame: usize, event: AnimationEvent) {
        self.events.entry(frame).or_default().push(event);
    }

    /// Get all events for a specific frame
    pub fn get_events(&self, frame: usize) -> Option<&Vec<AnimationEvent>> {
        self.events.get(&frame)
    }
}

/// Types of events that can be triggered by animations
#[derive(Debug, Clone)]
pub enum AnimationEvent {
    /// Play a sound effect by name
    PlaySound(String),
    /// Spawn a visual effect by name
    SpawnEffect(String),
    /// Custom event with string identifier (for game-specific logic)
    Custom(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_clip_frame_count() {
        let clip = AnimationClip::new(0, 9, 10.0);
        assert_eq!(clip.frame_count(), 10);

        let clip2 = AnimationClip::new(5, 10, 12.0);
        assert_eq!(clip2.frame_count(), 6);
    }

    #[test]
    fn test_animation_controller_add_and_play() {
        let mut controller = AnimationController::new();
        let mut state = AnimationState::default();

        controller.add_animation("idle", AnimationClip::new(0, 3, 8.0));
        controller.add_animation("run", AnimationClip::new(4, 9, 12.0));

        controller.play("idle", &mut state);
        assert_eq!(controller.current_animation, "idle");
        assert_eq!(state.current_frame, 0);
        assert!(state.playing);
    }

    #[test]
    fn test_animation_events() {
        let mut events = AnimationEvents::new();
        events.add_event(5, AnimationEvent::PlaySound("footstep.ogg".to_string()));
        events.add_event(5, AnimationEvent::SpawnEffect("dust".to_string()));

        let frame_events = events.get_events(5).unwrap();
        assert_eq!(frame_events.len(), 2);
    }
}
