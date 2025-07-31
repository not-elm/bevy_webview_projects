# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Core Development

- `cargo build` - Build the bevy_webview_wry crate
- `cargo test` - Run tests for this crate
- `cargo run --example <example_name>` - Run specific examples (e.g., `simple`, `ipc_command`, `embedding`)
- `cargo clippy` - Run linting checks
- `cargo doc --open` - Generate and open documentation

### Testing Examples

- `cargo run --example simple` - Basic webview window example
- `cargo run --example embedding` - Embedded webview in Bevy window
- `cargo run --example ipc_command` - IPC command communication example
- `cargo run --example ipc_trigger` - IPC trigger-based communication
- `cargo run --example child_window` - Child window webview example
- `cargo run --example emit_event_to_webview` - Event emission to webview

## Architecture Overview

### Core Functionality

`bevy_webview_wry` is the main crate providing webview functionality using the `wry` library for Bevy applications. It enables:

1. **Window Conversion** - Transform existing Bevy windows into webview windows
2. **Child Windows** - Create separate webview windows (requires `child_window` feature)  
3. **Embedded WebView** - Embed webview directly within Bevy windows (experimental)

### Key Components

**Plugin System:**
- `WebviewWryPlugin` - Main plugin that orchestrates all webview functionality
- Feature-based plugin loading with conditional compilation
- Integration with `bevy_child_window` for multi-window support

**Components:**
- `Webview` - Component that converts entities with windows into webview windows
- `IpcHandlers` - Component that defines available commands for each webview

**Resources:**
- `WryLocalRoot` - Configures local asset directory (defaults to "ui" under assets)

### Feature Flags

- `child_window` (default) - Enables separate webview windows
- `api` - Provides API plugins for webview communication  
- `hot-reload` - Enables webview hot-reloading during development

### Asset Management

- UI assets should be placed in `assets/ui/` directory (configurable via `local_root`)
- Assets are served from the local directory for security
- JavaScript API available as `Window.__FLURX__` in webview context

### Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Windows  | ✅ | Full support |
| macOS    | ✅ | Full support with Objective-C bindings |
| Linux (X11) | ✅ | Full support with GTK |
| Linux (Wayland) | ❌ | Not supported |

### Development Notes

**Version Compatibility:**
- Built for Bevy 0.16+ and bevy_flurx 0.12+
- Uses Rust edition 2024
- Breaking changes in v0.4.0+ switched from EventReader/EventEmitter to Trigger system

**IPC Communication:**
- Uses `bevy_flurx` for reactive programming patterns
- Supports both synchronous Action Commands and asynchronous Async Commands
- Commands are defined using procedural macros from `bevy_flurx_ipc_macro`

**Testing:**
- Run `cargo test` after making changes to verify functionality
- Test examples with `cargo run --example <name>` to ensure UI integration works
- Use `cargo clippy` to check for linting issues

**Hot Reload:**
- Enable `hot-reload` feature for development
- WebView components support hot reload during development
- Rust components require restart for changes