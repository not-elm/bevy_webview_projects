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
    /// const monitor = await window.__FLURX__.monitor.primary();
    /// ```
    MonitorPrimaryPlugin,
    command: primary
);

#[command(id = "FLURX|monitor::primary")]
fn primary(WebviewEntity(entity): WebviewEntity) -> Action<(), Option<Monitor>> {
    once::run(primary_system).with(entity).omit_input().with(())
}

//noinspection DuplicatedCode
fn primary_system(
    In(entity): In<Entity>,
    parent: Query<&EmbedWithin>,
    web_views: NonSend<WinitWindows>,
) -> Option<Monitor> {
    let entity = if let Ok(parent) = parent.get(entity) {
        parent.0
    } else {
        entity
    };

    web_views.get_window(entity)?.primary_monitor().map(|m| {
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
