use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::{In, MonitorSelection};
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to set current window position into center from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().center();
    /// ```
    WebWindowCenterPlugin,
    command: center
);

#[command(id = "FLURX|web_window::center")]
fn center(In(args): In<String>) -> Action<String> {
    once::run(system).with(args)
}

fn system(In(identifier): In<String>, mut web_views: WebWinitWindowParams) {
    let Some(mut window) = web_views.bevy_window_mut(&identifier) else {
        return;
    };
    window.position.center(MonitorSelection::Current);
}
