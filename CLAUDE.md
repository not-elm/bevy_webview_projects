# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Core Development

- `cargo build` - Build all crates in the workspace
- `cargo test --workspace` - Run all tests across all crates
- `cargo run --example <example_name>` - Run specific examples (e.g., `simple`, `ipc_command`, `embedding`)
- `cargo clippy --workspace` - Run linting checks with workspace-specific rules
- `cargo doc --workspace --open` - Generate and open documentation

### Package Management

- `cd tool/api && pnpm install` - Install TypeScript API dependencies
- `cd tool/api && pnpm build` - Build the TypeScript API client package
- `pnpm setup` - Install pnpm globally and dependencies (as defined in tool/api/package.json)

### Testing Individual Components

- `cargo test -p bevy_webview_wry` - Test specific crate
- `cargo run -p bevy_webview_wry --example simple` - Run example from specific crate
- `cargo check --workspace` - Fast syntax and type checking

## Architecture Overview

### Workspace Structure

This is a Rust workspace containing multiple interconnected crates that provide webview integration for Bevy applications:

**Core Crates:**
- `bevy_webview_wry` - Main crate providing webview functionality using wry
- `bevy_webview_core` - Core webview logic and components
- `bevy_flurx_ipc` - Inter-process communication between Bevy and webview
- `bevy_flurx_ipc_macro` - Procedural macros for IPC command generation
- `bevy_flurx_api` - API layer for webview-to-Bevy communication

**TypeScript Tooling:**
- `tool/api/` - TypeScript client library for webview integration

### Integration Architecture

**Bevy + WebView Integration:**
The library enables two primary integration patterns:
1. **Window Conversion** - Transform existing Bevy windows into webview windows
2. **Child Windows** - Create separate webview windows (requires `child_window` feature)
3. **Embedded WebView** - Embed webview directly within Bevy windows (experimental)

**IPC Communication:**
Uses `bevy_flurx` for reactive programming patterns with two command types:
- **Action Commands** - Synchronous commands that return immediately
- **Async Commands** - Asynchronous commands that can await Bevy ECS operations

**API System:**
Provides modular APIs accessible from webview JavaScript:
- File system operations (`fs` feature)
- Dialog boxes (`dialog` feature)
- Monitor information (`monitor` feature)
- OS information (`os` feature)
- Notifications (`notification` feature)
- HTTP requests (`http` feature)
- Window management (`web_window` feature)
- Clipboard access (`clipboard` feature)

### Key Components

**Plugin System:**
- `WebviewWryPlugin` - Main plugin that orchestrates all webview functionality
- Feature-based plugin loading with conditional compilation
- Integration with `bevy_child_window` for multi-window support

**Resource Management:**
- `WryLocalRoot` - Configures local asset directory (defaults to "ui" under assets)
- Assets served from configurable local directory for security

**Entity-Component Architecture:**
- `Webview` component converts entities with windows into webview windows
- `IpcHandlers` component defines available commands for each webview
- Uses Bevy's trigger system for event communication (v0.4.0+)

### Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Windows  | ✅ | Full support |
| macOS    | ✅ | Full support |
| Linux (X11) | ✅ | Full support |
| Linux (Wayland) | ❌ | Not supported |

### Development Notes

**Feature Flags:**
- `child_window` (default) - Enables separate webview windows
- `api` - Provides API plugins for webview communication
- `hot-reload` - Enables webview hot-reloading during development

**Version Compatibility:**
- Built for Bevy 0.16+ and bevy_flurx 0.12+
- Uses Rust edition 2024
- Breaking changes in v0.4.0 switched from EventReader/EventEmitter to Trigger system

**Asset Organization:**
- UI assets should be placed in `assets/ui/` directory (configurable via `local_root`)
- JavaScript API available as npm package `bevy_flurx_api` or via `Window.__FLURX__`
- Examples include complete HTML/JS/CSS for webview UI demonstrations