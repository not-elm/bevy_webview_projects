//! Defines the ipc commands and the queue to execute them.

use crate::component::{IpcHandlers, WebviewEntity};
use bevy::prelude::{
    App, Commands, Entity, Event, Plugin, Query, Reflect, ReflectDeserialize, ReflectSerialize,
    Res, Resource, Update,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// The ipc commands that exists only one in the [`World`](bevy::prelude::World).
#[derive(Resource, Clone, Default)]
pub struct IpcCommands(Arc<Mutex<Vec<IpcCommand>>>);

impl IpcCommands {
    /// Push the [`IpcCommand`] into queue.
    ///
    /// The pushed command is automatically executed and output as [`IpcResolveEvent`].
    #[inline(always)]
    pub fn push(&self, command: IpcCommand) {
        self.0.lock().unwrap().push(command);
    }

    #[inline(always)]
    pub(crate) fn take_commands(&self) -> Vec<IpcCommand> {
        self.0
            .try_lock()
            .map(|mut guard| std::mem::take(&mut *guard))
            .unwrap_or_default()
    }
}

/// The ipc command to execute.
///
/// [`IpcHandler`](crate::prelude::IpcHandler) must be spawned in the world to run this command.
#[derive(Deserialize, Debug)]
pub struct IpcCommand {
    /// The entity is attached to [`IpcHandlers`] that executes this command.
    pub entity: Entity,

    /// The command info passed from `javascript`.
    pub payload: Payload,
}

/// The command info passed from `javascript`.
#[derive(Deserialize, Debug)]
pub struct Payload {
    /// `ipc-command-id` to call.
    pub id: String,

    /// The serialized args passed from javascript.
    ///
    /// None if no arguments are passed from javascript.
    pub args: Option<String>,

    /// This value is used when waiting for IPC asynchronously.
    ///
    /// You need to create `resolve_id` in  `javascript` and then need to can call `Promise::resolve` with id.
    pub resolve_id: usize,
}

impl Payload {
    /// Deserializes arguments passed from Javascript.
    ///
    /// ## Panics
    ///
    /// Panics if deserialization fails or arguments does not exist.
    pub fn deserialize_args<Args>(&self) -> bevy::prelude::In<Args>
    where
        Args: DeserializeOwned,
    {
        let args = serde_json::from_str::<Args>(self.args.as_ref().unwrap()).unwrap_or_else(|e| {
            panic!(
                "failed deserialize ipc args type<{}>error:\n {e}",
                std::any::type_name::<Args>()
            )
        });
        bevy::prelude::In(args)
    }
}

/// The event signals the end of ipc processing.
#[derive(Event, Eq, PartialEq, Clone, Serialize, Deserialize, Reflect, Debug)]
#[reflect(Serialize, Deserialize)]
pub struct IpcResolveEvent {
    /// The entity attached to [`IpcHandlers`] that execute ipc.
    pub entity: Entity,

    /// The id used to resolve asynchronous ipc.
    pub resolve_id: usize,

    /// The serialized output value.
    pub output: String,
}

/// The common webview for IPC communication between `Webview` and `bevy`.
pub(crate) struct FlurxIpcCommandPlugin;

impl Plugin for FlurxIpcCommandPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WebviewEntity>()
            .add_event::<IpcResolveEvent>()
            .init_resource::<IpcCommands>()
            .add_systems(Update, receive_ipc_commands);
    }
}

fn receive_ipc_commands(
    mut commands: Commands,
    ipc_commands: Res<IpcCommands>,
    handlers: Query<&IpcHandlers>,
) {
    for cmd in ipc_commands.take_commands() {
        let Ok(handlers) = handlers.get(cmd.entity) else {
            continue;
        };
        if let Some(ipc_fn) = handlers.get(&cmd.payload.id) {
            ipc_fn(&mut commands, cmd);
        }
    }
}
