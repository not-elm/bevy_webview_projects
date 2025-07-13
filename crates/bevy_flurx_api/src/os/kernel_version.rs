use crate::macros::api_plugin;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to get the kernel version from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const version: string | null = await window.__FLURX__.os.kernelVersion();
    /// ```
    OsKernelVersionPlugin,
    command: kernel_version
);

#[command(id = "FLURX|os::kernel_version")]
fn kernel_version() -> Action<(), Option<String>> {
    once::run(sysinfo::System::kernel_version).with(())
}
