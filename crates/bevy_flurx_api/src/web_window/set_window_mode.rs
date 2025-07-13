use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::{In, MonitorSelection, VideoModeSelection};
use bevy::window::WindowMode;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use serde::Deserialize;

api_plugin!(
    /// You'll be able to set fullscreen state of the window decorations from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().setDecorations(true);
    /// ```
    WebWindowSetWindowModePlugin,
    command: set_window_mode
);

type Args = (String, VideoModeState);

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum VideoModeState {
    Fullscreen,
    Borderless,
    Windowed,
}

#[command(id = "FLURX|web_window::set_window_mode")]
fn set_window_mode(In(args): In<Args>) -> Action<Args> {
    once::run(system).with(args)
}

fn system(In(args): In<Args>, mut web_views: WebWinitWindowParams) {
    let Some(mut window) = web_views.bevy_window_mut(&args.0) else {
        return;
    };
    window.mode = match args.1 {
        VideoModeState::Fullscreen => {
            WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current)
        }
        VideoModeState::Borderless => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
        VideoModeState::Windowed => WindowMode::Windowed,
    };
}
