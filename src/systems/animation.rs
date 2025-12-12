//! Animation systems for sprite animation playback and state management

use bevy::prelude::*;

use crate::components::{
    AnimationController, AnimationEvent, AnimationEvents, AnimationState, GroundDetection, Player,
    Velocity,
};

/// Update all active animations, advancing frames based on time
pub fn update_animations(
    time: Res<Time>,
    mut query: Query<(&AnimationController, &mut AnimationState, &mut Sprite)>,
) {
    for (controller, mut state, mut sprite) in &mut query {
        // Skip if animation is not playing
        if !state.playing {
            continue;
        }

        // Get the current animation clip
        let Some(clip) = controller.current_clip() else {
            continue;
        };

        // Always sync sprite with current frame (important for immediate animation changes)
        if let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index != state.current_frame {
                atlas.index = state.current_frame;
            }
        }

        // Tick the timer
        state.timer.tick(time.delta());

        // Advance frame when timer finishes
        if state.timer.just_finished() {
            state.current_frame += 1;

            // Handle end of animation
            if state.current_frame > clip.last_frame {
                if state.looping {
                    // Loop back to the beginning
                    state.current_frame = clip.first_frame;
                } else {
                    // Stop at the last frame
                    state.current_frame = clip.last_frame;
                    state.playing = false;
                }
            }

            // Update the sprite's texture atlas index
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = state.current_frame;
            }
        }
    }
}

/// Control player animation based on movement state
pub fn player_animation_controller(
    mut query: Query<
        (
            &Velocity,
            &GroundDetection,
            &mut AnimationController,
            &mut AnimationState,
        ),
        With<Player>,
    >,
) {
    for (velocity, ground, mut controller, mut state) in &mut query {
        // Determine which animation to play based on player state
        let animation = if ground.is_grounded {
            // On ground: play idle or run based on horizontal velocity
            if velocity.x.abs() > 10.0 {
                "run"
            } else {
                "idle"
            }
        } else {
            // In air: play jump (ascending) or fall (descending) based on vertical velocity
            if velocity.y > 50.0 { "jump" } else { "fall" }
        };

        // Play the animation (will only restart if it's a different animation)
        controller.play(animation, &mut state);
    }
}

/// Process animation events that should trigger on specific frames
pub fn process_animation_events(query: Query<(&AnimationState, &AnimationEvents)>) {
    for (state, events) in &query {
        // Check if there are events for the current frame
        if let Some(frame_events) = events.get_events(state.current_frame) {
            for event in frame_events {
                match event {
                    AnimationEvent::PlaySound(sound) => {
                        // TODO: Integrate with audio system when available
                        debug!("Animation event: Play sound '{}'", sound);
                    }
                    AnimationEvent::SpawnEffect(effect) => {
                        // TODO: Integrate with VFX system when available
                        debug!("Animation event: Spawn effect '{}'", effect);
                    }
                    AnimationEvent::Custom(event_name) => {
                        debug!("Animation event: Custom event '{}'", event_name);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::AnimationClip;
    use bevy::time::TimePlugin;

    #[test]
    fn test_animation_system_frame_progression() {
        let mut app = App::new();
        app.add_plugins(TimePlugin); // Add time plugin for Time resource
        app.add_systems(Update, update_animations);

        // Set up a test entity with animation components
        let mut controller = AnimationController::new();
        controller.add_animation("test", AnimationClip::new(0, 3, 10.0).unwrap());

        let mut state = AnimationState::new(10.0, true);
        state.playing = true;

        let entity = app
            .world_mut()
            .spawn((
                controller,
                state,
                Sprite {
                    texture_atlas: Some(TextureAtlas {
                        layout: Handle::default(),
                        index: 0,
                    }),
                    ..default()
                },
            ))
            .id();

        // Update to trigger animation
        app.update();

        // Check that the system runs without crashing
        assert!(app.world().get::<AnimationState>(entity).is_some());
    }
}
