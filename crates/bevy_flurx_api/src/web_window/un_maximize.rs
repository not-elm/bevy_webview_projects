use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to un-maximize a window from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().unMaximize();
    /// ```
    WebWindowUnMaximizePlugin,
    command: un_maximize
);

#[command(id = "FLURX|web_window::un_maximize")]
fn un_maximize(In(args): In<String>) -> Action<String> {
    once::run(system).with(args)
}

fn system(In(identifier): In<String>, web_views: WebWinitWindowParams) {
    let Some(window) = web_views.winit_window(&identifier) else {
        return;
    };
    window.set_maximized(false);
}
