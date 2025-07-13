use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use winit::dpi::PhysicalPosition;

api_plugin!(
    /// You'll be able to obtain the window inner position.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const position: PhysicalPosition | null = await window.__FLURX__.Webview.current().outerPosition();
    /// ```
    WebWindowOuterPositionPlugin,
    command: outer_position
);

type Args = String;

#[command(id = "FLURX|web_window::outer_position")]
fn outer_position(In(args): In<Args>) -> Action<Args, Option<PhysicalPosition<i32>>> {
    once::run(system).with(args)
}

fn system(In(args): In<Args>, web_views: WebWinitWindowParams) -> Option<PhysicalPosition<i32>> {
    let window = web_views.winit_window(&args)?;
    window.outer_position().ok()
}
