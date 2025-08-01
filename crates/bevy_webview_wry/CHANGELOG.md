## v0.5.0

[Release notes](https://github.com/not-elm/bevy_webview_projects/releases/tag/v0.5.0)

### Breaking Changes

- Reexport `bevy_flurx` from `bevy_webview_wry` crate.
    - This is a measure to prevent version differences in bevy_flurx.

### Features

- Update `bevy_flurx` version to `0.12.0`.
- Support hot-reloading feature flag to enable hot-reloading of webviews.

### Bugfixes

- Fixed an issue where the embedded webview position would sometimes be incorrect when the window moved.

## v0.4.0

### Breaking Changes

- Changed the way to send events rom the webview to Bevy: now uses Trigger instead of EventReader
    - examples: [./examples/ipc_trigger.rs](./examples/ipc_trigger.rs)
- Changed the way to send events from Bevy to the webview: now uses Trigger instead of EventEmitter
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