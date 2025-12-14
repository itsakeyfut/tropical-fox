# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tropical Fox is a 2D action platformer built with Bevy 0.17.3 (Rust game engine). The game features sprite-based animation, physics, and is designed with a modular plugin architecture inspired by games like Celeste and Hollow Knight.

## Build and Development Commands

```bash
# Run the game (development mode with dynamic linking for faster iteration)
cargo run --features bevy-dev

# Run the game (standard mode)
cargo run

# Format code (MANDATORY before commits)
cargo fmt

# Lint code (MANDATORY before commits)
cargo clippy

# Run tests
cargo test

# Release build
cargo build --release
```

## Architecture

### Plugin Structure
The game uses Bevy's plugin system for modularity. Plugins are organized in `src/plugins/`:

- **CorePlugin** - Camera setup, window configuration, global resources
- **AnimationPlugin** - Sprite animation using benimator
- **PlayerPlugin** - Player input, movement, physics

### Game State Machine
Defined in `src/game_state.rs`:
- `GameState`: Loading -> Title -> WorldMap -> InGame -> Paused/GameOver
- `InGameState` (SubState of InGame): StagePlay, BossRoom, StageTransition

### Module Structure
```
src/
├── main.rs           # Entry point, plugin registration
├── game_state.rs     # State machine definitions
├── error.rs          # thiserror-based error types
├── config/           # RON configuration loading
├── components/       # ECS components (animation, etc.)
├── systems/          # Bevy systems (physics, player, animation)
├── plugins/          # Bevy plugins
├── resources/        # Global game resources
├── events/           # Custom Bevy events
└── debug/            # Debug mode features
```

### Configuration System
Game data is defined in RON files under `assets/config/`:
- `game_settings.ron` - Window, physics, player movement parameters
- `players.ron` - Player character definitions and animation paths
- `bosses.ron` - Boss character definitions and animation paths
- `enemies.ron` - Enemy type definitions with stats, AI, and behaviors
- `animation/*.ron` - Animation clip definitions

RON reference guide: `memo/ron_format_guide.md`

### Error Handling Philosophy
The game uses graceful degradation - errors are logged but fallback values are preferred over crashes. See `src/error.rs` for the error type hierarchy.

## Key Dependencies

- **bevy** (0.17.3) - Core game engine
- **bevy_rapier2d** - 2D physics
- **bevy_ecs_tilemap** - Tilemap rendering
- **bevy_kira_audio** - Audio playback
- **benimator** - Sprite animation
- **bevy_egui** / **bevy-inspector-egui** - Debug UI
- **serde** / **ron** - Configuration serialization

## Workflow Commands

Custom slash commands are available:
- `/impl <issue-number>` - Start implementing a GitHub issue
- `/finish [files...]` - Run checks, commit, and create PR

## Development Guidelines

- All code comments and documentation must be in English
- PRs must be written in Japanese
- Never commit directly to `main` - use feature branches (`feat/`, `fix/`, `refactor/`, `docs/`)
- Keep PRs under 100 lines of diff when possible
- Run `cargo fmt`, `cargo clippy`, `cargo test`, and `cargo build --release` before committing
