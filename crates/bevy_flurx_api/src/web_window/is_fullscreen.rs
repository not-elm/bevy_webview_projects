use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to get a focus state from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().isFocused();
    /// ```
    WebWindowIsFullscreenPlugin,
    command: is_fullscreen
);

#[command(id = "FLURX|web_window::is_fullscreen")]
fn is_fullscreen(In(args): In<String>) -> Action<String, bool> {
    once::run(system).with(args)
}

fn system(In(identifier): In<String>, web_views: WebWinitWindowParams) -> bool {
    let Some(window) = web_views.winit_window(&identifier) else {
        return false;
    };
    window.fullscreen().is_some()
}
