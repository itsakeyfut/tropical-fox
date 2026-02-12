use bevy::prelude::*;
use tropical_fox_common::{Collider, Player};

use crate::components::LevelGoal;
use crate::events::LevelCompleteEvent;

/// Checks AABB overlap between two colliders
#[inline]
fn check_aabb_overlap(
    transform_a: &Transform,
    collider_a: &Collider,
    transform_b: &Transform,
    collider_b: &Collider,
) -> bool {
    let pos_a = transform_a.translation.truncate();
    let pos_b = transform_b.translation.truncate();

    let a_min = pos_a + collider_a.offset - collider_a.size / 2.0;
    let a_max = pos_a + collider_a.offset + collider_a.size / 2.0;
    let b_min = pos_b + collider_b.offset - collider_b.size / 2.0;
    let b_max = pos_b + collider_b.offset + collider_b.size / 2.0;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

/// Detects when player reaches a goal entity
pub fn detect_goal_reached(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    goal_query: Query<(&Transform, &Collider, &LevelGoal)>,
    mut events: MessageWriter<LevelCompleteEvent>,
) {
    let Ok((player_transform, player_collider)) = player_query.single() else {
        return;
    };

    for (goal_transform, goal_collider, level_goal) in goal_query.iter() {
        // Create a larger trigger area for goals (easier to reach)
        let trigger_collider = Collider::new(goal_collider.size * 1.5);

        if check_aabb_overlap(
            player_transform,
            player_collider,
            goal_transform,
            &trigger_collider,
        ) {
            info!("Player reached goal!");
            events.write(LevelCompleteEvent {
                next_level: level_goal.next_level.clone(),
            });
        }
    }
}

/// Handles level completion events
pub fn handle_level_complete(
    mut events: MessageReader<LevelCompleteEvent>,
    mut next_state: ResMut<NextState<tropical_fox_common::GameState>>,
) {
    for event in events.read() {
        info!("Level complete!");
        if let Some(next) = &event.next_level {
            info!("Next level: {}", next);
            // TODO: Load next level (future enhancement)
            // For now, just return to world map
        } else {
            // No next level, return to world map
            info!("Returning to world map");
            next_state.set(tropical_fox_common::GameState::WorldMap);
        }
    }
}
