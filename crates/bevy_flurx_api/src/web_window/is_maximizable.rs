use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use winit::window::WindowButtons;

api_plugin!(
    /// You'll be able to get a focus state from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().isMaximizable();
    /// ```
    WebWindowIsMaximizablePlugin,
    command: is_maximizable
);

#[command(id = "FLURX|web_window::is_maximizable")]
fn is_maximizable(In(args): In<String>) -> Action<String, bool> {
    once::run(system).with(args)
}

fn system(In(identifier): In<String>, web_views: WebWinitWindowParams) -> bool {
    let Some(window) = web_views.winit_window(&identifier) else {
        return false;
    };
    window.enabled_buttons().contains(WindowButtons::MAXIMIZE)
}
