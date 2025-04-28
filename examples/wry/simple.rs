//! Minimum example showing how to create a webview in the window.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_webview_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WebviewWryPlugin::default()
        ))
        .add_systems(Startup, spawn_webview)
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    // Converts the `Window` attached the entity into a webview window. 
    commands
        .entity(window.single().expect("Parent window not found"))
        .insert(Webview::Uri(WebviewUri::new("https://bevyengine.org/")));
}

