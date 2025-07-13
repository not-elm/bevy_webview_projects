use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to minimize a window from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().maximize();
    /// ```
    WebWindowMinimizePlugin,
    command: minimize
);

#[command(id = "FLURX|web_window::minimize")]
fn minimize(In(args): In<String>) -> Action<String> {
    once::run(system).with(args)
}

fn system(In(identifier): In<String>, web_views: WebWinitWindowParams) {
    let Some(window) = web_views.winit_window(&identifier) else {
        return;
    };
    window.set_minimized(true);
}
