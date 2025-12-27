# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

DejaVu is a TUI (Text User Interface) duplicate file finder written in Rust. It helps users identify and manage duplicate media files (images and videos) through an interactive terminal interface with visual previews.

## Build and Run Commands

```bash
# Build the project (development)
cargo build

# Build optimized release version
cargo build --release

# Run the application
cargo run -- /path/to/scan

# Run release binary
./target/release/DejaVu /path/to/scan

# Run tests (when implemented)
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Architecture

This is an early-stage Rust project currently in initial setup phase. The planned architecture includes:

### Core Components (Planned)

1. **File Scanner**: Recursive directory traversal system for media file discovery
2. **Duplicate Detection Engine**: Content-based comparison for identifying identical and similar files
3. **TUI Interface**: Terminal-based interactive user interface
4. **Preview System**: Image thumbnail generation and video metadata display
5. **Selection Manager**: Interactive system for resolving duplicates with keyboard controls

### Expected Dependencies

The project will likely need:
- TUI framework: `ratatui` or `crossterm` for terminal UI
- Image processing: `image` crate for thumbnails and comparison
- File system: `walkdir` or similar for traversal
- Hashing: cryptographic or perceptual hashing for duplicate detection
- Video metadata: libraries for video file information extraction

## Current Status

- **Phase**: Initial prototype
- **Implementation**: Only "Hello, World!" exists in src/main.rs
- **Next Steps**: Implement file scanning, duplicate detection logic, and TUI framework

## Design Philosophy

- Retro TUI aesthetic for a modern problem
- Keyboard-driven interaction (no mouse required)
- Visual previews directly in terminal
- Efficient batch operations for file management
