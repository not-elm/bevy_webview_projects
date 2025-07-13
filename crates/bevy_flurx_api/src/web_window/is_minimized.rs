use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to get the window is minimizable from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().isMinimizable();
    /// ```
    WebWindowIsMinimizedPlugin,
    command: is_minimized
);

#[command(id = "FLURX|web_window::is_minimized")]
fn is_minimized(In(args): In<String>) -> Action<String, bool> {
    once::run(system).with(args)
}

fn system(In(identifier): In<String>, web_views: WebWinitWindowParams) -> bool {
    let Some(window) = web_views.winit_window(&identifier) else {
        return false;
    };
    window.is_minimized().unwrap_or_default()
}
