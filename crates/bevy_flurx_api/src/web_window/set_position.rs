use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::math::IVec2;
use bevy::prelude::{In, WindowPosition};
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use winit::dpi::PhysicalPosition;

api_plugin!(
    /// You'll be able to set the window position.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().setPosition({ x: 0, y: 0 });
    /// ```
    WebWindowSetPositionPlugin,
    command: set_position
);

type Args = (String, PhysicalPosition<i32>);

#[command(id = "FLURX|web_window::set_position")]
fn set_position(In(args): In<Args>) -> Action<Args> {
    once::run(system).with(args)
}

fn system(In(args): In<Args>, mut web_views: WebWinitWindowParams) {
    let Some(mut window) = web_views.bevy_window_mut(&args.0) else {
        return;
    };
    window.position = WindowPosition::At(IVec2::new(args.1.x, args.1.y));
}
