use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use winit::dpi::PhysicalSize;

api_plugin!(
    /// You'll be able to set current window position into center from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().center();
    /// ```
    WebWindowInnerSizePlugin,
    command: inner_size
);

#[command(id = "FLURX|web_window::inner_size")]
fn inner_size(In(args): In<String>) -> Action<String, Option<PhysicalSize<u32>>> {
    once::run(system).with(args)
}

fn system(
    In(identifier): In<String>,
    mut web_views: WebWinitWindowParams,
) -> Option<PhysicalSize<u32>> {
    let window = web_views.bevy_window_mut(&identifier)?;

    Some(PhysicalSize::new(
        window.physical_width(),
        window.physical_height(),
    ))
}
