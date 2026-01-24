//! Goal/stage clear systems
//!
//! Handles goal collision detection and stage transitions

use bevy::prelude::*;
use tropical_fox_common::{Collider, InGameState, Player};

use crate::components::Goal;

/// Check for player collision with goal and trigger stage transition
pub fn check_goal_collision(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    goal_query: Query<&Transform, With<Goal>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    let Some((player_transform, player_collider)) = player_query.iter().next() else {
        return;
    };

    let player_pos = player_transform.translation;
    let player_half_size = player_collider.size / 2.0;

    for goal_transform in goal_query.iter() {
        let goal_pos = goal_transform.translation;
        let goal_half_size = Vec2::new(16.0, 16.0); // Half of 32x32 goal size

        // AABB collision detection
        let collision = player_pos.x + player_half_size.x > goal_pos.x - goal_half_size.x
            && player_pos.x - player_half_size.x < goal_pos.x + goal_half_size.x
            && player_pos.y + player_half_size.y > goal_pos.y - goal_half_size.y
            && player_pos.y - player_half_size.y < goal_pos.y + goal_half_size.y;

        if collision {
            info!("🏁 Player reached goal! Transitioning to StageTransition state");
            next_state.set(InGameState::StageTransition);
            return;
        }
    }
}

/// Handle stage transition state (placeholder for now)
pub fn handle_stage_transition(mut next_state: ResMut<NextState<InGameState>>) {
    info!("Handling stage transition...");

    // For now, just go back to StagePlay
    // In a full implementation, this would:
    // - Show transition animation
    // - Load next level
    // - Update CurrentLevel resource
    // - Spawn new level

    info!("Transition complete, returning to StagePlay");
    next_state.set(InGameState::StagePlay);
}
