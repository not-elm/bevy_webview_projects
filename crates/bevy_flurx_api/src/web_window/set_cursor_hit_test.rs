use crate::error::ApiResult;
use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to set whether the cursor hit from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().setCursorHitTest(true);
    /// ```
    WebWindowSetCursorHitTestPlugin,
    command: set_cursor_hit_test
);

type Args = (String, bool);

#[command(id = "FLURX|web_window::set_cursor_hit_test")]
fn set_cursor_hit_test(In(args): In<Args>) -> Action<Args, ApiResult> {
    once::run(system).with(args)
}

fn system(In(args): In<Args>, web_views: WebWinitWindowParams) -> ApiResult {
    let Some(window) = web_views.winit_window(&args.0) else {
        return Ok(());
    };
    window.set_cursor_hittest(args.1)?;
    Ok(())
}
