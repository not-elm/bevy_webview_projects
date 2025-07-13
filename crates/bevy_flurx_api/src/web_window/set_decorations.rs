use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to set visible of the window decorations from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().setDecorations(true);
    /// ```
    WebWindowSetDecorationsPlugin,
    command: set_decorations
);

type Args = (String, bool);

#[command(id = "FLURX|web_window::set_decorations")]
fn set_decorations(In(args): In<Args>) -> Action<Args> {
    once::run(system).with(args)
}

fn system(In(args): In<Args>, mut web_views: WebWinitWindowParams) {
    let Some(mut window) = web_views.bevy_window_mut(&args.0) else {
        return;
    };
    window.decorations = args.1;
}
