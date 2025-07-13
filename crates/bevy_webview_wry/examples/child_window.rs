//! This example demonstrates how to create a child window with a webview.

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_webview_wry::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, WebviewWryPlugin::default()))
        .add_systems(Startup, spawn_child_window)
        .run();
}

fn spawn_child_window(mut commands: Commands, window: Query<Entity, With<PrimaryWindow>>) {
    commands.spawn((
        Window {
            title: "Child Window".to_string(),
            resolution: WindowResolution::new(500., 500.),
            ..default()
        },
        Webview::Uri(WebviewUri::new("https://bevyengine.org/")),
        ParentWindow(window.single().expect("Parent window not found")),
    ));
}
