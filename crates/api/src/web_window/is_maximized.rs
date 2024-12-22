use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

api_plugin!(
    /// You'll be able to get a focus state from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().isMaximized();
    /// ```
    WebWindowIsMaximizedPlugin,
    command: is_maximized
);

#[command(id = "FLURX|web_window::is_maximized", internal)]
fn is_maximized(In(args): In<String>) -> Action<String, bool> {
    once::run(system).with(args)
}

fn system(
    In(identifier): In<String>,
    web_views: WebWinitWindowParams,
) -> bool {
    let Some(window) = web_views.winit_window(&identifier) else {
        return false;
    };
    window.is_maximized()
}