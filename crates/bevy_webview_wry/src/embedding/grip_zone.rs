use crate::embedding::CurrentMoving;
use crate::prelude::{DragEntered, GripZone, WryWebViews};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::util::WryResultLog;
use bevy::ecs::system::SystemParam;
use bevy::input::common_conditions::input_just_released;
#[cfg(not(target_os = "linux"))]
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::winit::WinitWindows;
use bevy_flurx_ipc::ipc_trigger::IpcTriggerExt;
use bevy_webview_core::bundle::embedding::{Bounds, EmbedWithin};
use serde::Deserialize;
use wry::raw_window_handle::HasWindowHandle;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use wry::raw_window_handle::RawWindowHandle;
#[cfg(target_os = "windows")]
use wry::WebViewExtWindows;

pub struct GripZonePlugin;

impl Plugin for GripZonePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_ipc_trigger::<OnGripGrab>("FLURX|grip::grab")
            .add_ipc_trigger::<OnGripRelease>("FLURX|grip::release")
            .add_systems(Update, (
                drag.run_if(any_with_component::<CurrentMoving>),
                resize_grip_zone,
                all_remove_current_moving.run_if(input_just_released(MouseButton::Left).or(on_event::<DragEntered>)),
            ).run_if(any_with_component::<GripZone>))
            .add_observer(apply_drag_start)
            .add_observer(apply_drag_end);

        #[cfg(target_os = "linux")]
        app.add_ipc_event::<OnGribDrag>("FLURX|grip::drag");
    }
}

fn resize_grip_zone(
    webviews: Query<(Entity, &GripZone), Changed<GripZone>>,
    wry_webviews: NonSend<WryWebViews>,
) {
    for (entity, grip_zone) in webviews.iter() {
        if let Some(webview) = wry_webviews.0.get(&entity) {
            if let Err(e) = webview.evaluate_script(&format!("window.__FLURX__.gripZoneHeight={}", grip_zone.0)) {
                bevy::log::warn!("Failed to grip zone height: {}", e);
            }
        }
    }
}

#[derive(SystemParam)]
struct MouseDelta<'w, 's> {
    #[cfg(not(target_os = "linux"))]
    er: EventReader<'w, 's, MouseMotion>,
    /// I was testing on Ubuntu 24.04 ARM64 in Parallels, but `MouseMotion` was getting clearly abnormal coordinates,
    /// so I switched to getting delta from webview.
    #[cfg(target_os = "linux")]
    er: EventReader<'w, 's, IpcEvent<OnGribDrag>>,
}

impl MouseDelta<'_, '_> {
    #[cfg(not(target_os = "linux"))]
    pub fn delta(&mut self) -> Option<Vec2> {
        self
            .er
            .read()
            .map(|event| event.delta)
            .reduce(|d1, d2| d1 + d2)
    }

    #[cfg(target_os = "linux")]
    pub fn delta(&mut self) -> Option<Vec2> {
        self
            .er
            .read()
            .map(|event| {
                Vec2::new(event.payload.x, event.payload.y)
            })
            .reduce(|d1, d2| d1 + d2)
    }
}

fn drag(
    mut mouse_delta: MouseDelta,
    mut webviews: Query<(&mut Bounds, &mut CurrentMoving, &EmbedWithin), With<CurrentMoving>>,
    windows: Query<&Window>,
) {
    let Some(delta) = mouse_delta.delta() else {
        return;
    };

    for (mut bounds, mut moving, parent) in webviews.iter_mut() {
        let Ok(window) = windows.get(parent.0) else {
            continue;
        };
        let window_size = Vec2::new(window.width(), window.height());
        moving.0 = delta;
        move_bounds(&mut bounds, moving.0, window_size, None);
    }
}

fn all_remove_current_moving(mut commands: Commands, views: Query<Entity, With<CurrentMoving>>) {
    for entity in views.iter() {
        commands.entity(entity).remove::<CurrentMoving>();
    }
}

fn move_bounds(
    bounds: &mut Bounds,
    offset: Vec2,
    window_size: Vec2,
    toolbar_height: Option<f32>,
) {
    let max = toolbar_height
        .map(|height| Vec2::new(0., height))
        .unwrap_or_default();
    let max_pos = (window_size - bounds.size).max(Vec2::ZERO);
    let new_pos = if cfg!(target_os = "macos") {
        Vec2::new(offset.x, -offset.y)
    } else {
        offset
    };
    bounds.position = (bounds.position + new_pos).min(max_pos).max(max);
}

#[derive(Deserialize, Event)]
struct OnGripGrab {
    x: f32,
    y: f32,
}

#[cfg(target_os = "linux")]
#[derive(Deserialize)]
struct OnGribDrag {
    x: f32,
    y: f32,
}

fn apply_drag_start(
    trigger: Trigger<OnGripGrab>,
    mut commands: Commands,
    wry_webviews: NonSend<WryWebViews>,
    winit_windows: NonSend<WinitWindows>,
    webviews: Query<&EmbedWithin>,
) {
    let webview_entity = trigger.target();
    let Ok(EmbedWithin(window_entity)) = webviews.get(webview_entity) else {
        return;
    };

    commands
        .entity(webview_entity)
        .insert(CurrentMoving(Vec2::new(trigger.x, trigger.y)));

    bring_to_front(webview_entity, window_entity, &wry_webviews, &winit_windows);
}

fn bring_to_front(
    window_entity: Entity,
    webview_entity: &Entity,
    wry_webviews: &WryWebViews,
    winit_windows: &WinitWindows,
) {
    let Some(_webview) = wry_webviews.0.get(webview_entity) else {
        return;
    };
    let Some(window_handle) = winit_windows
        .get_window(window_entity)
        .and_then(|w| w.window_handle().ok())
        .map(|h| h.as_raw())
    else {
        return;
    };
    match window_handle {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Win32(handle) => {
            _webview.reparent(handle.hwnd.get()).output_log_if_failed();
        }
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        RawWindowHandle::AppKit(_) => {
            use wry::WebViewExtMacOS;
            use objc2::rc::Retained;
            _webview.reparent(Retained::into_raw(_webview.ns_window())).output_log_if_failed();
        }
        _ => {}
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Event)]
struct OnGripRelease {
    __FLURX__grip_release: u8,
}

fn apply_drag_end(
    trigger: Trigger<OnGripRelease>,
    mut commands: Commands,
) {
    let webview_entity = trigger.target();
    commands.entity(webview_entity).remove::<CurrentMoving>();
}

#[cfg(test)]
mod tests {
    use crate::embedding::grip_zone::move_bounds;
    use crate::prelude::Bounds;
    use bevy::prelude::*;

    #[test]
    fn stop_bottom_right() {
        let mut bounds = new_bounds();
        move_bounds(
            &mut bounds,
            Vec2::new(110., 110.),
            Vec2::new(100., 100.),
            None,
        );
        #[cfg(target_os = "macos")]
        assert_eq!(bounds.position, Vec2::new(95., 0.));
        #[cfg(not(target_os = "macos"))]
        assert_eq!(bounds.position, Vec2::new(95., 90.));
    }

    fn new_bounds() -> Bounds {
        Bounds {
            position: Vec2::new(5., 10.),
            size: Vec2::new(5., 10.),
            ..default()
        }
    }
}
