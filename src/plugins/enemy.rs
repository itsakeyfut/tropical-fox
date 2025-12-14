//! Enemy plugin
//!
//! Handles enemy spawning, AI systems, and enemy-specific behavior.

use bevy::prelude::*;

use crate::combat::{DamageEvent, DeathEvent, EnemyHealth, Health, Invincibility};
use crate::components::{
    AnimationClip, AnimationController, AnimationState, ChaseAI, Collider, ContactDamage, Enemy,
    EnemyAI, EnemyProjectile, EnemyStats, FlyingAI, FlyingPattern, Gravity, Ground, PatrolAI,
    Player, ProjectileShooter, Velocity,
};
use crate::config::{
    AIBehaviorType, EnemiesConfig, FlyingPatternConfig, load_animation_config_optional,
    load_enemies_config_optional,
};
use crate::game_state::GameState;
use crate::resources::CharacterAssets;

/// Resource holding enemy configuration
#[derive(Resource, Debug)]
pub struct EnemyConfig(pub EnemiesConfig);

/// Resource for tracking game score
#[derive(Resource, Debug, Default)]
pub struct Score {
    /// Current score value
    pub value: u32,
}

impl Score {
    /// Add points to the score
    pub fn add(&mut self, points: u32) {
        self.value += points;
        info!("Score: {} (+{})", self.value, points);
    }
}

/// Plugin that manages enemy entities and AI systems
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Load enemy configuration
        let enemies_config = load_enemies_config_optional("assets/config/enemies.ron");
        app.insert_resource(EnemyConfig(enemies_config));

        // Add score resource
        app.insert_resource(Score::default());

        // Spawn test enemies when entering InGame state
        app.add_systems(OnEnter(GameState::InGame), spawn_test_enemies);

        // AI systems (run in Update)
        app.add_systems(
            Update,
            (
                // AI behavior systems
                patrol_ai_system,
                chase_ai_system,
                flying_ai_system,
                // Attack systems
                contact_damage_cooldown_system,
                contact_damage_system,
                projectile_shooter_system,
                projectile_movement_system,
                projectile_collision_system,
                // Visual updates
                enemy_animation_controller,
                enemy_facing_system,
            )
                .run_if(in_state(GameState::InGame)),
        );

        // Enemy ground collision (run in FixedUpdate with physics)
        app.add_systems(
            FixedUpdate,
            enemy_ground_collision.run_if(in_state(GameState::InGame)),
        );

        // Enemy death handling (must run after combat systems)
        app.add_systems(
            Update,
            enemy_death_system
                .after(crate::combat::death_system)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

/// Spawn test enemies for development
/// Ground is at Y = -200 with height 32, so ground surface is at Y = -184
fn spawn_test_enemies(
    mut commands: Commands,
    config: Res<EnemyConfig>,
    character_assets: Option<Res<CharacterAssets>>,
) {
    // Ground surface Y position (top of ground platform)
    const GROUND_SURFACE_Y: f32 = -184.0;

    // Spawn an ant (patrol enemy)
    // Collider height: 31, spawn at ground + half height
    if let Some(ant_config) = config.0.get("ant") {
        let spawn_y = GROUND_SURFACE_Y + ant_config.collider.size.1 / 2.0;
        spawn_enemy(
            &mut commands,
            "ant",
            Vec2::new(-100.0, spawn_y),
            ant_config,
            character_assets.as_deref(),
        );
        info!("Spawned ant at (-100, {})", spawn_y);
    }

    // Spawn a bat (flying enemy) - flies in the air
    if let Some(bat_config) = config.0.get("bat") {
        spawn_enemy(
            &mut commands,
            "bat",
            Vec2::new(-150.0, 50.0),
            bat_config,
            character_assets.as_deref(),
        );
        info!("Spawned bat at (-150, 50)");
    }

    // Spawn a bear (chase enemy)
    // Collider height: 63, spawn at ground + half height
    if let Some(bear_config) = config.0.get("bear") {
        let spawn_y = GROUND_SURFACE_Y + bear_config.collider.size.1 / 2.0;
        spawn_enemy(
            &mut commands,
            "bear",
            Vec2::new(200.0, spawn_y),
            bear_config,
            character_assets.as_deref(),
        );
        info!("Spawned bear at (200, {})", spawn_y);
    }

    // Spawn a piranha (projectile shooter)
    // Collider height: 45, spawn at ground + half height
    if let Some(piranha_config) = config.0.get("piranha") {
        let spawn_y = GROUND_SURFACE_Y + piranha_config.collider.size.1 / 2.0;
        spawn_enemy(
            &mut commands,
            "piranha",
            Vec2::new(-250.0, spawn_y),
            piranha_config,
            character_assets.as_deref(),
        );
        info!("Spawned piranha at (-250, {})", spawn_y);
    }
}

/// Spawn an enemy entity from configuration
pub fn spawn_enemy(
    commands: &mut Commands,
    enemy_type: &str,
    position: Vec2,
    config: &crate::config::EnemyTypeConfig,
    character_assets: Option<&CharacterAssets>,
) {
    let collider_size = Vec2::new(config.collider.size.0, config.collider.size.1);
    // Use 2x scale for display
    let display_size = collider_size * 2.0;

    let mut entity = commands.spawn((
        Transform::from_translation(position.extend(0.0)),
        Visibility::default(),
        Enemy::new(enemy_type),
        EnemyStats {
            move_speed: config.stats.move_speed,
            damage: config.stats.damage,
            knockback_force: config.stats.knockback_force,
            score_value: config.stats.score_value,
        },
        Health::new(config.stats.health),
        EnemyHealth,
        Velocity::default(),
        Collider::new(collider_size),
        ContactDamage::new(config.stats.damage, config.stats.knockback_force, 0.5),
        Name::new(format!("Enemy_{}", config.name)),
    ));

    // Try to use character assets for sprite
    if let Some(assets) = character_assets {
        if let Some(atlas) = assets.get(enemy_type) {
            // Create animation controller from config
            let animation_config_path = format!(
                "assets/graphics/characters/enemies/{}/{}_animations.ron",
                enemy_type, enemy_type
            );
            let (animation_controller, animation_state) =
                if let Some(anim_config) = load_animation_config_optional(&animation_config_path) {
                    match AnimationController::try_from(anim_config) {
                        Ok(controller) => controller.with_initial_state(true),
                        Err(e) => {
                            warn!(
                                "Failed to create AnimationController for {}: {}",
                                enemy_type, e
                            );
                            create_fallback_enemy_animation()
                        }
                    }
                } else {
                    create_fallback_enemy_animation()
                };

            entity.insert(Sprite {
                image: atlas.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas.layout.clone(),
                    index: 0,
                }),
                custom_size: Some(display_size),
                ..default()
            });
            entity.insert(animation_controller);
            entity.insert(animation_state);
        } else {
            // Fallback to colored placeholder
            entity.insert(Sprite {
                color: get_enemy_color(enemy_type),
                custom_size: Some(display_size),
                ..default()
            });
        }
    } else {
        // Fallback to colored placeholder
        entity.insert(Sprite {
            color: get_enemy_color(enemy_type),
            custom_size: Some(display_size),
            ..default()
        });
    }

    // Add gravity if applicable
    if config.has_gravity {
        entity.insert(Gravity::default());
    }

    // Add AI component based on behavior type
    let ai = create_ai_component(&config.ai, position);
    entity.insert(ai);

    // Add projectile shooter if enabled
    if config.projectile.enabled {
        entity.insert(ProjectileShooter::new(
            config.projectile.damage,
            config.projectile.speed,
            config.projectile.fire_rate,
            config.projectile.range,
        ));
    }
}

/// Create fallback animation controller for enemies without config
fn create_fallback_enemy_animation() -> (AnimationController, AnimationState) {
    let mut controller = AnimationController::new();
    // Simple single-frame animation
    if let Ok(clip) = AnimationClip::new(0, 0, 1.0) {
        controller.add_animation("idle", clip);
    }
    controller.current_animation = "idle".to_string();
    controller.with_initial_state(true)
}

/// Get a placeholder color for an enemy type
fn get_enemy_color(enemy_type: &str) -> Color {
    match enemy_type {
        "ant" => Color::srgb(0.8, 0.3, 0.2),         // Reddish-brown
        "bat" => Color::srgb(0.5, 0.2, 0.5),         // Purple
        "bear" => Color::srgb(0.6, 0.4, 0.2),        // Brown
        "piranha" => Color::srgb(0.2, 0.6, 0.3),     // Green-ish
        "slime" => Color::srgb(0.2, 0.8, 0.3),       // Green
        "goblin" => Color::srgb(0.6, 0.3, 0.1),      // Brown
        "ghost" => Color::srgba(0.7, 0.7, 0.9, 0.7), // Translucent blue
        "skeleton" => Color::srgb(0.9, 0.9, 0.85),   // Off-white
        "wisp" => Color::srgb(0.9, 0.9, 0.3),        // Yellow
        _ => Color::srgb(0.8, 0.2, 0.2),             // Default red
    }
}

/// Create an AI component from configuration
fn create_ai_component(ai_config: &crate::config::AIConfig, origin: Vec2) -> EnemyAI {
    match ai_config.behavior {
        AIBehaviorType::Idle => EnemyAI::Idle,
        AIBehaviorType::Patrol => EnemyAI::Patrol(PatrolAI::horizontal(
            ai_config.patrol_distance,
            ai_config.wait_time,
            origin,
        )),
        AIBehaviorType::Chase => EnemyAI::Chase(ChaseAI::new(
            ai_config.detection_range,
            ai_config.attack_range,
        )),
        AIBehaviorType::Flying => {
            let pattern = match &ai_config.flying_pattern {
                Some(FlyingPatternConfig::SineWave {
                    amplitude,
                    frequency,
                    speed,
                }) => FlyingPattern::SineWave {
                    amplitude: *amplitude,
                    frequency: *frequency,
                    speed: *speed,
                },
                Some(FlyingPatternConfig::Circle { radius, speed }) => FlyingPattern::Circle {
                    radius: *radius,
                    speed: *speed,
                },
                Some(FlyingPatternConfig::Hover { range, speed }) => FlyingPattern::Hover {
                    range: *range,
                    speed: *speed,
                },
                Some(FlyingPatternConfig::Figure8 {
                    width,
                    height,
                    speed,
                }) => FlyingPattern::Figure8 {
                    width: *width,
                    height: *height,
                    speed: *speed,
                },
                None => FlyingPattern::default(),
            };
            EnemyAI::Flying(FlyingAI::new(pattern, origin))
        }
    }
}

/// Patrol AI system - moves enemy between waypoints
fn patrol_ai_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut EnemyAI, &EnemyStats), With<Enemy>>,
) {
    for (transform, mut velocity, mut ai, stats) in query.iter_mut() {
        if let EnemyAI::Patrol(ref mut patrol) = *ai {
            let current_pos = transform.translation.truncate();
            let target_pos = patrol.current_target();
            let distance = current_pos.distance(target_pos);

            const ARRIVAL_THRESHOLD: f32 = 5.0;

            if distance < ARRIVAL_THRESHOLD {
                // At waypoint - wait then advance
                velocity.x = 0.0;
                patrol.wait_timer += time.delta_secs();

                if patrol.wait_timer >= patrol.wait_time {
                    patrol.advance();
                }
            } else {
                // Move toward target
                let direction = (target_pos - current_pos).normalize();
                velocity.x = direction.x * stats.move_speed;
            }
        }
    }
}

/// Chase AI system - enemy chases player when in detection range
fn chase_ai_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Velocity, &mut EnemyAI, &EnemyStats), With<Enemy>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    for (enemy_transform, mut velocity, mut ai, stats) in enemy_query.iter_mut() {
        if let EnemyAI::Chase(ref mut chase) = *ai {
            let enemy_pos = enemy_transform.translation.truncate();
            let distance = enemy_pos.distance(player_pos);

            if distance < chase.detection_range {
                // Player detected
                chase.is_chasing = true;
                chase.lost_sight_timer = 0.0;

                if distance > chase.attack_range {
                    // Chase player
                    let direction = (player_pos - enemy_pos).normalize();
                    velocity.x = direction.x * stats.move_speed;
                } else {
                    // In attack range - stop
                    velocity.x = 0.0;
                }
            } else if chase.is_chasing {
                // Lost sight of player
                chase.lost_sight_timer += time.delta_secs();

                if chase.lost_sight_timer >= chase.lose_aggro_time {
                    chase.is_chasing = false;
                    velocity.x = 0.0;
                }
            } else {
                velocity.x = 0.0;
            }
        }
    }
}

/// Flying AI system - handles flying movement patterns
fn flying_ai_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut EnemyAI), With<Enemy>>,
) {
    for (mut transform, mut velocity, mut ai) in query.iter_mut() {
        if let EnemyAI::Flying(ref mut flying) = *ai {
            flying.time += time.delta_secs();

            match &flying.pattern {
                FlyingPattern::SineWave {
                    amplitude,
                    frequency,
                    speed,
                } => {
                    // Move horizontally and oscillate vertically
                    let x = flying.origin.x + flying.time * speed;
                    let y = flying.origin.y + (flying.time * frequency).sin() * amplitude;
                    transform.translation.x = x;
                    transform.translation.y = y;
                    // Set velocity for collision purposes
                    velocity.x = *speed;
                    velocity.y = (flying.time * frequency).cos() * amplitude * frequency;
                }
                FlyingPattern::Circle { radius, speed } => {
                    let angle = flying.time * speed;
                    let x = flying.origin.x + angle.cos() * radius;
                    let y = flying.origin.y + angle.sin() * radius;
                    transform.translation.x = x;
                    transform.translation.y = y;
                    velocity.x = -angle.sin() * radius * speed;
                    velocity.y = angle.cos() * radius * speed;
                }
                FlyingPattern::Hover { range, speed } => {
                    let offset = (flying.time * speed).sin() * range;
                    transform.translation.y = flying.origin.y + offset;
                    velocity.x = 0.0;
                    velocity.y = (flying.time * speed).cos() * range * speed;
                }
                FlyingPattern::Figure8 {
                    width,
                    height,
                    speed,
                } => {
                    let t = flying.time * speed;
                    let x = flying.origin.x + (t * 2.0).sin() * width;
                    let y = flying.origin.y + t.sin() * height;
                    transform.translation.x = x;
                    transform.translation.y = y;
                    velocity.x = (t * 2.0).cos() * width * speed * 2.0;
                    velocity.y = t.cos() * height * speed;
                }
            }
        }
    }
}

/// Update contact damage cooldown timers
fn contact_damage_cooldown_system(time: Res<Time>, mut query: Query<&mut ContactDamage>) {
    for mut contact in query.iter_mut() {
        contact.tick(time.delta_secs());
    }
}

/// Contact damage system - deals damage when enemy touches player
fn contact_damage_system(
    player_query: Query<(Entity, &Transform, &Collider, Option<&Invincibility>), With<Player>>,
    mut enemy_query: Query<(&Transform, &Collider, &mut ContactDamage, &Enemy)>,
    mut damage_events: MessageWriter<DamageEvent>,
) {
    let Ok((player_entity, player_transform, player_collider, invincibility)) =
        player_query.single()
    else {
        return;
    };

    // Skip if player is invincible
    if invincibility.is_some() {
        return;
    }

    let player_pos = player_transform.translation.truncate();

    for (enemy_transform, enemy_collider, mut contact, enemy) in enemy_query.iter_mut() {
        if !contact.can_damage() {
            continue;
        }

        let enemy_pos = enemy_transform.translation.truncate();

        if check_collision(player_pos, player_collider, enemy_pos, enemy_collider) {
            // Calculate knockback direction (push player away from enemy)
            let direction = (player_pos - enemy_pos).normalize_or_zero();
            let knockback = direction * contact.knockback_force;

            damage_events.write(DamageEvent {
                target: player_entity,
                damage: contact.damage,
                knockback,
                attacker: None,
            });

            contact.reset_cooldown();

            info!(
                "{} dealt {} contact damage to player",
                enemy.enemy_type, contact.damage
            );
        }
    }
}

/// Projectile shooter system - enemies fire projectiles at player
fn projectile_shooter_system(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut shooter_query: Query<(&Transform, &mut ProjectileShooter, &Enemy)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    for (enemy_transform, mut shooter, enemy) in shooter_query.iter_mut() {
        shooter.tick(time.delta_secs());

        if !shooter.can_fire() {
            continue;
        }

        let enemy_pos = enemy_transform.translation.truncate();
        let distance = enemy_pos.distance(player_pos);

        if distance <= shooter.range {
            // Fire projectile toward player
            let direction = (player_pos - enemy_pos).normalize();
            let velocity = direction * shooter.projectile_speed;

            commands.spawn((
                Transform::from_translation(enemy_pos.extend(0.0)),
                Visibility::default(),
                Sprite {
                    color: Color::srgb(1.0, 0.5, 0.0), // Orange projectile
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                EnemyProjectile::new(shooter.damage, 100.0, velocity, 5.0),
                Collider::new(Vec2::new(8.0, 8.0)),
                Name::new(format!("Projectile_{}", enemy.enemy_type)),
            ));

            shooter.reset_timer();

            info!("{} fired projectile at player", enemy.enemy_type);
        }
    }
}

/// Projectile movement system
fn projectile_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut EnemyProjectile)>,
) {
    for (entity, mut transform, mut projectile) in query.iter_mut() {
        // Update position
        transform.translation.x += projectile.velocity.x * time.delta_secs();
        transform.translation.y += projectile.velocity.y * time.delta_secs();

        // Update lifetime
        projectile.lifetime.tick(time.delta());

        if projectile.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Projectile collision system - check if projectiles hit player
fn projectile_collision_system(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform, &Collider, Option<&Invincibility>), With<Player>>,
    projectile_query: Query<(Entity, &Transform, &Collider, &EnemyProjectile)>,
    mut damage_events: MessageWriter<DamageEvent>,
) {
    let Ok((player_entity, player_transform, player_collider, invincibility)) =
        player_query.single()
    else {
        return;
    };

    // Skip if player is invincible
    if invincibility.is_some() {
        return;
    }

    let player_pos = player_transform.translation.truncate();

    for (proj_entity, proj_transform, proj_collider, projectile) in projectile_query.iter() {
        let proj_pos = proj_transform.translation.truncate();

        if check_collision(player_pos, player_collider, proj_pos, proj_collider) {
            // Deal damage to player
            let knockback = projectile.velocity.normalize() * projectile.knockback;

            damage_events.write(DamageEvent {
                target: player_entity,
                damage: projectile.damage,
                knockback,
                attacker: Some(proj_entity),
            });

            // Destroy projectile
            commands.entity(proj_entity).despawn();

            info!("Projectile hit player for {} damage", projectile.damage);
        }
    }
}

/// Update enemy animations based on AI state and velocity
fn enemy_animation_controller(
    mut query: Query<
        (
            &Enemy,
            &Velocity,
            &EnemyAI,
            &mut AnimationController,
            &mut AnimationState,
        ),
        With<Enemy>,
    >,
) {
    for (enemy, velocity, ai, mut controller, mut state) in query.iter_mut() {
        let animation_name = match ai {
            EnemyAI::Idle => "idle",
            EnemyAI::Patrol(_) => {
                if velocity.x.abs() > 0.1 {
                    "run"
                } else {
                    "idle"
                }
            }
            EnemyAI::Chase(chase) => {
                if chase.is_chasing && velocity.x.abs() > 0.1 {
                    "run"
                } else {
                    "idle"
                }
            }
            EnemyAI::Flying(_) => {
                // Flying enemies use "fly" animation, fallback to "idle"
                if controller.animations.contains_key("fly") {
                    "fly"
                } else {
                    "idle"
                }
            }
        };

        // Only change animation if different from current
        if controller.current_animation != animation_name
            && controller.animations.contains_key(animation_name)
        {
            controller.play(animation_name, &mut state);
            info!(
                "Enemy {} animation changed to {}",
                enemy.enemy_type, animation_name
            );
        }
    }
}

/// Update enemy facing direction based on velocity
fn enemy_facing_system(mut query: Query<(&mut Enemy, &Velocity, &mut Sprite)>) {
    for (mut enemy, velocity, mut sprite) in query.iter_mut() {
        if velocity.x.abs() > 0.1 {
            enemy.facing_right = velocity.x > 0.0;
            // Handle sprites that face left by default
            if enemy.sprite_faces_left {
                sprite.flip_x = enemy.facing_right;
            } else {
                sprite.flip_x = !enemy.facing_right;
            }
        }
    }
}

/// Handle enemy death - add score and spawn effects
fn enemy_death_system(
    mut death_events: MessageReader<DeathEvent>,
    enemy_query: Query<(&EnemyStats, &Enemy)>,
    mut score: ResMut<Score>,
) {
    for event in death_events.read() {
        if let Ok((stats, enemy)) = enemy_query.get(event.entity) {
            // Add score
            score.add(stats.score_value);

            info!(
                "{} defeated! +{} points (Total: {})",
                enemy.enemy_type, stats.score_value, score.value
            );

            // TODO: Spawn death effect particles
            // TODO: Spawn drop items based on configuration
        }
    }
}

/// Ground collision system for enemies with gravity
fn enemy_ground_collision(
    mut enemy_query: Query<
        (&mut Transform, &mut Velocity, &Collider),
        (With<Enemy>, With<Gravity>),
    >,
    ground_query: Query<(&Transform, &Collider), (With<Ground>, Without<Enemy>)>,
) {
    for (mut enemy_transform, mut velocity, enemy_collider) in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation.truncate();

        for (ground_transform, ground_collider) in ground_query.iter() {
            let ground_pos = ground_transform.translation.truncate();

            // Check AABB collision
            if check_collision(enemy_pos, enemy_collider, ground_pos, ground_collider) {
                // Check if enemy is on top of the ground (falling down onto it)
                let enemy_bottom =
                    enemy_pos.y + enemy_collider.offset.y - enemy_collider.size.y / 2.0;
                let ground_top =
                    ground_pos.y + ground_collider.offset.y + ground_collider.size.y / 2.0;

                // Enemy is on ground if they're above or slightly overlapping with ground top
                if enemy_bottom <= ground_top + 5.0 && velocity.y <= 0.1 {
                    // Position enemy on top of the ground
                    enemy_transform.translation.y = ground_transform.translation.y
                        + ground_collider.size.y / 2.0
                        + enemy_collider.size.y / 2.0;

                    // Stop downward velocity
                    if velocity.y < 0.0 {
                        velocity.y = 0.0;
                    }
                }
            }
        }
    }
}

/// Check AABB collision between two colliders
#[inline]
fn check_collision(pos_a: Vec2, collider_a: &Collider, pos_b: Vec2, collider_b: &Collider) -> bool {
    let a_min = pos_a + collider_a.offset - collider_a.size / 2.0;
    let a_max = pos_a + collider_a.offset + collider_a.size / 2.0;
    let b_min = pos_b + collider_b.offset - collider_b.size / 2.0;
    let b_max = pos_b + collider_b.offset + collider_b.size / 2.0;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let mut score = Score::default();
        assert_eq!(score.value, 0);

        score.add(100);
        assert_eq!(score.value, 100);

        score.add(50);
        assert_eq!(score.value, 150);
    }

    #[test]
    fn test_collision_detection() {
        let collider = Collider::new(Vec2::new(32.0, 32.0));

        // Overlapping
        assert!(check_collision(
            Vec2::ZERO,
            &collider,
            Vec2::new(16.0, 0.0),
            &collider
        ));

        // Not overlapping
        assert!(!check_collision(
            Vec2::ZERO,
            &collider,
            Vec2::new(100.0, 0.0),
            &collider
        ));
    }
}
