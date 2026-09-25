# AGENTS.md

## Setup
- **Dev environment**: Nix flake with direnv (`use flake` in `.envrc`)
- **Toolchain**: Latest stable Rust, rustfmt, clippy, rust-analyzer included
- **Build**: Standard Cargo; no special setup needed

## Build & Run
```bash
cargo build          # Debug binary
cargo build --release
cargo run -- <args>  # Run with CLI args, e.g., `cargo run -- greet --name test`
```

## Project Structure
- **Entrypoint**: `src/main.rs` — CLI parser using clap
- **Commands**: `src/commands/mod.rs` — subcommand enum dispatches to modules under `src/commands/`
- **Binary name**: `thatproject` (from `Cargo.toml`)


## Subagents

- **tester**: Used to run rust tests. If no arguments are passed then just run all tests.