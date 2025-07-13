//! This library assists in implementing inter-process communication and is also part of [
//! `bevy_webview_wry`](../bevy_webview_wry/README.md).
//!
//! Use [`bevy_flurx`](https://github.com/not-elm/bevy_flurx) for interprocess communication.
//! Its provides a mechanism similar to coroutines, making it easy to implement asynchronous communication.

use crate::ipc_commands::FlurxIpcCommandPlugin;
use crate::prelude::IpcTriggerPlugin;
use bevy::prelude::{App, Plugin};
use bevy_flurx::FlurxPlugin;
pub use bevy_flurx_ipc_macro::command;

pub mod component;
pub mod ipc_commands;
pub mod ipc_trigger;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{FlurxIpcPlugin, component::*, ipc_commands::*, ipc_trigger::*};
    pub use bevy_flurx_ipc_macro::command;
    use serde::Serialize;

    pub fn to_string<O: Serialize>(output: O) -> String {
        serde_json::to_string(&output).expect("Failed to deserialize output value.")
    }
}

/// This webview assists in implementing inter-process communication.
///
/// Please see for [`module-level documentation`](crate).
pub struct FlurxIpcPlugin;

impl Plugin for FlurxIpcPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FlurxPlugin>() {
            app.add_plugins(FlurxPlugin);
        }

        app.add_plugins((FlurxIpcCommandPlugin, IpcTriggerPlugin));
    }
}
