//! Testing to pass async bevy_flurx_ipc-commands to the `ipc_handlers!`.

use bevy::prelude::*;
use bevy_flurx::action::{delay, once};
use bevy_flurx::task::ReactorTask;
use bevy_flurx_ipc::prelude::*;

#[command]
async fn pattern1() -> String {
    "hello".to_string()
}

#[command]
async fn pattern2(In(_): In<()>) {}

#[command]
async fn pattern3(_webview_entity: WebviewEntity) {}

#[command]
async fn pattern4(task: ReactorTask) -> String {
    task.will(Update, once::run(|| "hello".to_string())).await
}

#[command]
async fn pattern5(In(_args): In<()>, WebviewEntity(_entity): WebviewEntity) {}

#[command]
async fn pattern6(In(frames): In<usize>, task: ReactorTask) {
    task.will(Update, delay::frames().with(frames)).await;
}

#[command]
async fn pattern7(In(_args): In<()>, _task: ReactorTask) {}

#[command]
async fn pattern8(In(_args): In<()>, _entity: WebviewEntity, _task: ReactorTask) {}

#[command]
async fn pattern9(_entity: WebviewEntity, In(_args): In<()>, _task: ReactorTask) {}

#[command]
async fn pattern10(_task: ReactorTask, _entity: WebviewEntity, In(_args): In<()>) {}

#[command]
async fn pattern11(_entity: WebviewEntity, _task: ReactorTask, In(_args): In<()>) {}

#[command]
async fn pattern12(In(_args): In<()>, _task: ReactorTask, _entity: WebviewEntity) {}

fn main() {
    IpcHandlers::new([
        pattern1, pattern2, pattern3, pattern4, pattern5, pattern6, pattern7, pattern8, pattern9,
        pattern10, pattern11, pattern12,
    ]);
}
