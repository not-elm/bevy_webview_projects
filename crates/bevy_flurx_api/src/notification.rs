//! Provides the mechanism to access the system notification.

use crate::error::ApiResult;
use crate::macros::api_plugin;
use bevy::prelude::In;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use serde::Deserialize;

api_plugin!(
    /// You'll be able to send a notification from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.notification.send("message");
    /// ```
    NotificationSendPlugin,
    command: send
);

#[derive(Deserialize)]
struct Args {
    message: String,
    title: Option<String>,
    icon: Option<String>,
}

#[command(id = "FLURX|notification::send")]
fn send(In(args): In<Args>) -> Action<Args, ApiResult> {
    once::run(send_system).with(args)
}

fn send_system(In(args): In<Args>) -> ApiResult {
    let mut notification = notify_rust::Notification::default();
    if let Some(title) = args.title {
        notification.summary(&title);
    }
    if let Some(icon) = args.icon {
        notification.icon(&icon);
    }
    notification.body(&args.message);
    notification.show()?;
    Ok(())
}
