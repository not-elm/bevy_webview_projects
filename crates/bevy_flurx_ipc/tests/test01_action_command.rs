//!  Testing to define `bevy_flurx_ipc-command` via macro.

use bevy::prelude::*;
use bevy_flurx::action::once;
use bevy_flurx::prelude::{Action, ActionSeed};
use bevy_flurx_ipc::prelude::*;
use bevy_flurx_ipc_macro::command;

#[command]
fn pattern1() -> ActionSeed {
    once::run(|| {})
}

#[command]
fn pattern2(_: In<String>) -> ActionSeed {
    once::run(|| {})
}

#[command]
fn pattern3(_: WebviewEntity) -> ActionSeed {
    once::run(|| {})
}

#[command]
fn pattern4(_: In<String>, _: WebviewEntity) -> ActionSeed {
    once::run(|| {})
}

#[command]
fn pattern5(_: WebviewEntity, _: In<String>) -> ActionSeed {
    once::run(|| {})
}

#[command]
fn action_command(
    In(args): In<String>,
    entity: WebviewEntity,
) -> Action<(String, WebviewEntity), String> {
    once::run(|In(_): In<(String, WebviewEntity)>| "output is returned to Javascript".to_string())
        .with((args, entity))
}

fn main() {
    IpcHandlers::new([
        pattern1,
        pattern2,
        pattern3,
        pattern4,
        pattern5,
        action_command,
    ]);
}
