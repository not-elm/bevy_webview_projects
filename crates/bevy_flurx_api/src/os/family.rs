use crate::macros::api_plugin;
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;

api_plugin!(
    /// You'll be able to get a describing the family of the operating system from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.os.family();
    /// ```
    OsFamilyPlugin,
    command: family
);

#[command(id = "FLURX|os::family")]
fn family() -> Action<(), Option<&'static str>> {
    fn f() -> Option<&'static str> {
        if std::env::consts::FAMILY.is_empty() {
            None
        } else {
            Some(std::env::consts::FAMILY)
        }
    }
    once::run(f).with(())
}
