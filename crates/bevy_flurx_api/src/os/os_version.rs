use crate::macros::api_plugin;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to get the system version from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.os.longOsVersion();
    /// ```
    OsLongVersionPlugin,
    command: long_os_version
);

api_plugin!(
    /// You'll be able to get the system version from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.os.version();
    /// ```
    OsVersionPlugin,
    command: os_version
);

#[command(id = "FLURX|os::long_os_version")]
fn long_os_version() -> Action<(), Option<String>> {
    once::run(sysinfo::System::long_os_version).with(())
}

#[command(id = "FLURX|os::os_version")]
fn os_version() -> Action<(), Option<String>> {
    once::run(sysinfo::System::os_version).with(())
}
