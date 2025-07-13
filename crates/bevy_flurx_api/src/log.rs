//! Provides mechanism to output the logs.

use bevy::log;
use bevy::prelude::{App, Event, Plugin, Trigger};
use bevy_flurx_ipc::ipc_trigger::IpcTriggerExt;
use serde::Deserialize;

/// You will be able to output a massage to the console of the aa process.
///
/// ## Typescript Code Example
///
/// ```ts
/// window.__FLURX__.log.println("message")
/// ```
pub struct AllLogPlugins;

impl Plugin for AllLogPlugins {
    fn build(&self, app: &mut App) {
        app.add_ipc_trigger::<RequestPrintln>("FLURX|log::println")
            .add_ipc_trigger::<RequestLog>("FLURX|log::log")
            .add_observer(apply_println_api)
            .add_observer(apply_log_api);
    }
}

#[derive(Deserialize, Event)]
struct RequestPrintln {
    message: String,
}

#[derive(Deserialize, Event)]
struct RequestLog {
    message: String,
    level: RequestLogLevel,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum RequestLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

fn apply_println_api(trigger: Trigger<RequestPrintln>) {
    println!("{}", trigger.message);
}

fn apply_log_api(trigger: Trigger<RequestLog>) {
    let message = &trigger.message;
    match trigger.level {
        RequestLogLevel::Trace => log::trace!(message),
        RequestLogLevel::Debug => log::debug!(message),
        RequestLogLevel::Info => log::info!(message),
        RequestLogLevel::Warn => log::warn!(message),
        RequestLogLevel::Error => log::error!(message),
    }
}
