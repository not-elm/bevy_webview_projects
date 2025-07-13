use crate::macros::api_plugin;
use crate::monitor::{Monitor, PhysicalPosition, PhysicalSize};
use bevy::prelude::{Entity, In, NonSend, Query};
use bevy::winit::WinitWindows;
use bevy_flurx::action::{Action, once};
use bevy_flurx::prelude::OmitInput;
use bevy_flurx_ipc::component::WebviewEntity;
use bevy_flurx_ipc::prelude::*;
use bevy_webview_core::prelude::EmbedWithin;

api_plugin!(
    /// You'll be able to get a describing the monitor infos from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const monitors = await window.__FLURX__.monitor.availables();
    /// ```
    MonitorAvailablesPlugin,
    command: available_monitors
);

#[command(id = "FLURX|monitor::availables")]
fn available_monitors(WebviewEntity(entity): WebviewEntity) -> Action<(), Vec<Monitor>> {
    once::run(available_monitors_system)
        .with(entity)
        .omit_input()
        .with(())
}

//noinspection DuplicatedCode
fn available_monitors_system(
    In(entity): In<Entity>,
    parent: Query<&EmbedWithin>,
    web_views: NonSend<WinitWindows>,
) -> Vec<Monitor> {
    let entity = if let Ok(parent) = parent.get(entity) {
        parent.0
    } else {
        entity
    };
    let Some(win) = web_views.get_window(entity) else {
        return Vec::with_capacity(0);
    };
    win.available_monitors()
        .map(|m| {
            let size = m.size();
            let p = m.position();
            Monitor {
                name: m.name(),
                size: PhysicalSize {
                    width: size.width,
                    height: size.height,
                },
                position: PhysicalPosition { x: p.x, y: p.y },
                scale_factor: m.scale_factor(),
            }
        })
        .collect()
}
