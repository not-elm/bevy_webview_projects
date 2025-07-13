//! Testing to edit the id of  bevy_flurx_ipc-command.

use bevy_flurx_ipc::prelude::*;

#[command(id = "FLURX|TEST")]
async fn hello() -> String {
    "hello".to_string()
}

fn main() {
    IpcHandlers::new([hello]);
    assert_eq!(hello().id(), "FLURX|TEST");
}
