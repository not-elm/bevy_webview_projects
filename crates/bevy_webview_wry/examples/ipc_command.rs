//!  This example show how to use `action-command` and `task-command`.
//!
//! This commands allows you to receive input from the webview and return output there

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_flurx::prelude::*;
use bevy_flurx_ipc::prelude::*;
use bevy_webview_wry::prelude::*;
use std::path::PathBuf;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WebviewWryPlugin {
                local_root: PathBuf::from("ui").join("ipc_command"),
            },
        ))
        .add_systems(Startup, spawn_webview)
        .run();
}

#[derive(Component)]
struct Num(usize);

fn spawn_webview(mut commands: Commands, window: Query<Entity, With<PrimaryWindow>>) {
    commands
        .entity(window.single().expect("Parent window was not found"))
        .insert((
            Num(1),
            // Display `assets/ui/ipc_command/index.html` within the webview
            Webview::default(),
            IpcHandlers::new([action_command, async_command]),
        ));
}

/// Input args are optional.
#[command]
fn action_command(
    In(n): In<usize>, // The input from javascript
    WebviewEntity(entity): WebviewEntity,
) -> Action<(Entity, usize), usize> {
    once::run(fibonacci).with((entity, n))
}

/// As with `action-command`, input args are optional.
#[command]
async fn async_command(
    In(n): In<usize>, // The input from javascript
    WebviewEntity(entity): WebviewEntity,
    task: ReactorTask,
) -> usize {
    task.will(Update, once::run(fibonacci).with((entity, n)))
        .await
}

fn fibonacci(In((webview_entity, n)): In<(Entity, usize)>, mut views: Query<&mut Num>) -> usize {
    if let Ok(mut num) = views.get_mut(webview_entity) {
        let out = num.0 + n;
        num.0 = n;
        out
    } else {
        0
    }
}
