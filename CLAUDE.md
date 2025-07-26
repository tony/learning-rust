# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Commands

### Build
```bash
cargo build
cargo build --release
```

### Run
```bash
cargo run
cargo run --release
```

### Test
```bash
cargo test
cargo test -- --nocapture  # Show println! output
cargo test test_name        # Run specific test
```

### Lint and Format
```bash
cargo fmt
cargo clippy
```

## Architecture

This is a simple Rust binary crate that demonstrates tmux integration:

- **main.rs**: Contains the main application logic for finding and executing tmux commands
  - `find_tmux_path()`: Locates tmux binary in the system PATH or uses a custom path
  - `run_tmux_command()`: Executes tmux commands and returns output
  - Includes comprehensive unit tests using mock executables

- **lib.rs**: Contains a simple example library function `add()` with tests

The project uses:
- `rand` as a runtime dependency
- `tempfile` as a dev dependency for creating temporary directories in tests