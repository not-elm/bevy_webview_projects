//! This example demonstrates how to use `bevy_flurx_ipc`.

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_flurx_ipc::prelude::*;

#[derive(Resource)]
struct Count(usize);

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            LogPlugin::default(),
            FlurxPlugin,
            FlurxIpcPlugin,
        ))
        .insert_resource(Count(0))
        .add_systems(Startup, setup)
        .add_systems(Update, resolve_event)
        .run();
}

fn increment() -> ActionSeed<usize, usize> {
    once::run(|In(n): In<usize>, mut count: ResMut<Count>| {
        count.0 += n;
        count.0
    })
}

#[command]
async fn increment_command(In(n): In<usize>, task: ReactorTask) -> usize {
    task.will(Update, increment().with(n)).await
}

fn setup(mut commands: Commands, ipc_commands: Res<IpcCommands>) {
    let entity = commands.spawn(IpcHandlers::new([increment_command])).id();

    // This time, threads are treated as other processes.
    let ipc_commands = ipc_commands.clone();
    std::thread::spawn(move || {
        let mut count = 0;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            count += 1;
            ipc_commands.push(IpcCommand {
                entity,
                payload: Payload {
                    // Call `increment_command` command.
                    id: "increment_command".to_string(),
                    args: Some(format!("{count}")),
                    // ID to identify the caller
                    resolve_id: 0,
                },
            });
        }
    });
}

fn resolve_event(mut er: EventReader<IpcResolveEvent>) {
    for e in er.read() {
        info!("Resolved: {:?}", e);
    }
}
