//! This example show how to emit the event to a webview side.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_flurx_api::web_window::AllWebWindowPlugins;
use bevy_webview_wry::prelude::*;
use std::path::PathBuf;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AllWebWindowPlugins,
            WebviewWryPlugin {
                local_root: PathBuf::from("ui").join("event_emit")
            }
        ))
        .insert_resource(CountTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)))
        .add_ipc_trigger("count_event")
        .add_systems(Startup, spawn_webview)
        .add_systems(Update, emit_event)
        .run();
}

#[derive(Resource)]
struct CountTimer(Timer);

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    // Display `assets/ui/event_emit/index.html` within the webview.
    commands.entity(window.single().expect("Window wasn't found")).insert(Webview::default());
}

fn emit_event(
    mut timer: ResMut<CountTimer>,
    mut views: Query<&mut EventEmitter>,
    mut count: Local<usize>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).finished() {
        *count += 1;
        for mut emitter in views.iter_mut() {
            emitter.emit("count_event", serde_json::json!({
                "count" : *count
            }));
        }
    }
}