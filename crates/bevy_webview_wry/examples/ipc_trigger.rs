//! This Example show how to listen the message from webview.
//!
//! Logs out messages emitted from text boxes.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_flurx_ipc::prelude::*;
use bevy_webview_wry::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WebviewWryPlugin {
                local_root: PathBuf::from("ui").join("ipc_trigger"),
            },
        ))
        .add_ipc_trigger::<MessageFromWebview>("message")
        .add_systems(Startup, spawn_webview)
        .add_observer(apply_webview_message)
        .run();
}

fn spawn_webview(mut commands: Commands, window: Query<Entity, With<PrimaryWindow>>) {
    commands
        .entity(window.single().expect("Window wasn't found"))
        .insert(Webview::default());
}

#[derive(Deserialize, Debug, Event)]
struct MessageFromWebview {
    message: String,
}

fn apply_webview_message(trigger: Trigger<MessageFromWebview>) {
    info!("message from webview: {}", trigger.message);
}
