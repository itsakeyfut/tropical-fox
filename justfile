# Update local main branch
new:
    git checkout main && git fetch && git pull origin main

dev-check:
    cargo check --features bevy-dev

dev-build:
    cargo build --features bevy-dev

dev-run:
    cargo run --features bevy-dev