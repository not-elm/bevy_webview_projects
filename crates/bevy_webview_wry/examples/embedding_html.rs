//! This example demonstrates how to create a webview with embedding html content.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_webview_wry::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, WebviewWryPlugin::default()))
        .add_systems(Startup, insert_webview)
        .run();
}

fn insert_webview(mut commands: Commands, window: Query<Entity, With<PrimaryWindow>>) {
    commands
        .entity(window.single().expect("Parent window not found"))
        .insert(Webview::Html(
            "<html><body><h1>Hello world!</h1></body></html>".to_string(),
        ));
}
