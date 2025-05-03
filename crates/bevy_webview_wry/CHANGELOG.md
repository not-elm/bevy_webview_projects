## Unreleased

### Breaking Changes

- Replaced EventReader‑based event handling from the webView with triggers.
    - examples: [./examples/ipc_trigger.rs](./examples/ipc_trigger.rs)
- Replaced the event‑sending mechanism to the ebView from an EwentEmitter to triggers.
    - examples: [./examples/emit_event_to_webview.rs](./examples/emit_event_to_webview.rs)

### Features

- Upgrade bevy's version to v0.16.0

## v0.3.0

[Release notes](https://github.com/not-elm/bevy_webview_projects/releases/tag/v0.3.0)

### Breaking Changes

- reexport `ipc` and `api` from `bevy_webview_wry`.
- added `bevy_webview_core` crate and moved Webview components there.

### Features

- Support for Linux(X11).

### Bugfix

- fixed api build path
- stop mystery navigation when executing ipc-command.

## v0.2.0

[Release notes](https://github.com/not-elm/bevy_webview_projects/releases/tag/v0.2.0)

## v0.1.0

First release!
Please feel free to report me at [issue](https://github.com/not-elm/bevy_webview_projects/issues).