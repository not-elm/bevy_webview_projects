//! Minimum example showing how to create a webview as child in the window.
//!
//! This feature is still experimental and may not work as expected.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_webview_wry::prelude::*;
use std::path::PathBuf;

fn main() {
    let mut app = App::new();

    #[cfg(target_os = "macos")]
    app.insert_resource(ClearColor(Color::NONE));

    app
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    #[cfg(target_os = "macos")]
                    composite_alpha_mode: bevy::window::CompositeAlphaMode::PostMultiplied,
                    ..default()
                }),
                ..default()
            }),
            WebviewWryPlugin {
                local_root: PathBuf::from("ui").join("embedding")
            }
        ))
        .add_systems(Startup, spawn_webview)
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.spawn((
        Webview::Uri(WebviewUri::default()),
        // Specifies the window entity to embed.
        EmbedWithin(window.single().expect("Parent window not found")),
        Resizable(true),
        // Grab the top of the webview and allow it to move.
        GripZone(10),
        Bounds {
            position: Vec2::new(100., 100.),
            size: Vec2::new(500., 500.),
            min_size: Vec2::new(100., 100.),
        },
    ));

    commands.spawn((
        Webview::Uri(WebviewUri::new("https://bevyengine.org/")),
        EmbedWithin(window.single().expect("Parent window not found")),
        Bounds {
            position: Vec2::new(700., 100.),
            size: Vec2::new(500., 500.),
            min_size: Vec2::new(100., 100.),
        },
    ));
}

