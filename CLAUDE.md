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

### Workspace Structure
The project uses a Cargo workspace with modular crates:

```
app/
├── tropical-fox/     # Main binary crate (entry point, core setup)
├── common/           # Shared types (game states, components, events, resources)
├── animation/        # Animation system plugin (benimator integration)
├── player/           # Player input, movement, physics plugin
├── combat/           # Combat system plugin
├── enemy/            # Enemy AI and behavior plugin
└── hot-asset/        # Hot-reload infrastructure for RON configs
```

**Dependency flow**: `tropical-fox` → domain crates (player, enemy, combat, animation) → `common`

### Plugin Architecture
Each domain crate exports a Bevy plugin registered in `app/tropical-fox/src/main.rs`:

- **CorePlugin** (`tropical-fox`) - Camera, window, global physics setup
- **AnimationPlugin** (`animation`) - Sprite animation via benimator
- **PlayerPlugin** (`player`) - Character control, physics, animation state
- **CombatPlugin** (`combat`) - Hitboxes, damage, health systems
- **EnemyPlugin** (`enemy`) - Enemy spawning, AI, boss mechanics
- **HotReloadPlugin** (`hot-asset`, debug only) - RON config file watching

### Game State Machine
Defined in `app/common/src/game_state.rs`:
- `GameState`: Loading → Title → WorldMap → InGame → Paused/GameOver
- `InGameState` (SubState of InGame): StagePlay, BossRoom, StageTransition

State transitions drive system scheduling and plugin activation.

### Configuration System
Game data is defined in RON files under `assets/config/`:
- `game_settings.ron` - Window, physics, player movement parameters
- `players.ron` - Player character definitions and animation paths
- `bosses.ron` - Boss character definitions and animation paths
- `enemies.ron` - Enemy type definitions with stats, AI, and behaviors

**Hot-reload**: In debug builds, RON config changes are detected via `notify` crate and automatically reloaded. Hot-reload handlers are in `app/tropical-fox/src/hot_reload_systems.rs`.

RON reference guide: `memo/ron_format_guide.md`

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

### Working with the Workspace

When adding new features, determine which crate should own the functionality:
- Shared types/states → `common`
- Character control → `player`
- Enemy behavior → `enemy`
- Visual effects/animation → `animation`
- Damage/health systems → `combat`
- Infrastructure/debugging → `hot-asset` or `tropical-fox`

Add cross-crate dependencies in the relevant `Cargo.toml` and import via `use tropical_fox_<crate>::...`
