use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to get  a webview window title from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const title: string | null = await window.__FLURX__.Webview.current().title();
    /// ```
    WebWindowTitlePlugin,
    command: title
);

#[command(id = "FLURX|web_window::title")]
fn title(In(args): In<String>) -> Action<String, Option<String>> {
    once::run(system).with(args)
}

fn system(In(identifier): In<String>, web_views: WebWinitWindowParams) -> Option<String> {
    let window = web_views.winit_window(&identifier)?;
    Some(window.title())
}
