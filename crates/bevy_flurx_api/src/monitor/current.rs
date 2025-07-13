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
    /// You'll be able to get the current monitor info from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const monitor = await window.__FLURX__.window.currentMonitor();
    /// ```
    MonitorCurrentPlugin,
    command: current_monitor
);

#[command(id = "FLURX|monitor::current")]
fn current_monitor(WebviewEntity(entity): WebviewEntity) -> Action<(), Option<Monitor>> {
    once::run(current_monitor_system)
        .with(entity)
        .omit_input()
        .with(())
}

//noinspection DuplicatedCode
fn current_monitor_system(
    In(entity): In<Entity>,
    parent: Query<&EmbedWithin>,
    web_views: NonSend<WinitWindows>,
) -> Option<Monitor> {
    let entity = if let Ok(parent) = parent.get(entity) {
        parent.0
    } else {
        entity
    };

    web_views.get_window(entity)?.current_monitor().map(|m| {
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
}
