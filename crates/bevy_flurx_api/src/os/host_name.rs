use crate::macros::api_plugin;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to get the host name from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const name: string | null = await window.__FLURX__.os.hostName();
    /// ```
    OsHostNamePlugin,
    command: host_name
);

#[command(id = "FLURX|os::host_name")]
fn host_name() -> Action<(), Option<String>> {
    once::run(sysinfo::System::host_name).with(())
}
