//! Provides the utility actions to read a system information.

mod family;
mod host_name;
mod kernel_version;
mod locale;
mod os_version;
mod system_name;

use crate::macros::api_plugin;
pub use crate::os::family::OsFamilyPlugin;
pub use crate::os::host_name::OsHostNamePlugin;
pub use crate::os::kernel_version::OsKernelVersionPlugin;
pub use crate::os::locale::OsLocalePlugin;
pub use crate::os::os_version::{OsLongVersionPlugin, OsVersionPlugin};
pub use crate::os::system_name::OsSystemNamePlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::PluginGroup;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

/// Allows you to use all os plugins.
///
/// ## Plugins
///
/// - [OsArchPlugin]
/// - [OsFamilyPlugin]
/// - [OsVersionPlugin]
/// - [OsLongVersionPlugin]
/// - [OsKernelVersionPlugin]
/// - [OsSystemNamePlugin]
/// - [OsHostNamePlugin]
/// - [OsLocalePlugin]
pub struct AllOsPlugins;
impl PluginGroup for AllOsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(OsArchPlugin)
            .add(OsFamilyPlugin)
            .add(OsVersionPlugin)
            .add(OsLongVersionPlugin)
            .add(OsKernelVersionPlugin)
            .add(OsSystemNamePlugin)
            .add(OsHostNamePlugin)
            .add(OsLocalePlugin)
    }
}

api_plugin!(
    /// You'll be able to get a describing the architecture of the CPU from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.os.arch();
    /// ```
    OsArchPlugin,
    command: arch
);

#[command(id = "FLURX|os::arch")]
fn arch() -> Action<(), &'static str> {
    fn f() -> &'static str {
        std::env::consts::ARCH
    }
    once::run(f).with(())
}
