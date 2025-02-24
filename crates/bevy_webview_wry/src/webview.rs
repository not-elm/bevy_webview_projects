//! Provides a mechanism to control the basic behavior of Webview.

use crate::webview::event_emitter::EventEmitterPlugin;
use crate::webview::handlers::WryHandlersPlugin;
use crate::webview::ipc_resolve::IpcResolvePlugin;
use crate::webview::load_webview::LoadWebviewPlugin;
use crate::webview::protocol::CustomProtocolPlugin;
use crate::webview::visible::VisiblePlugin;
use bevy::prelude::{App, Deref, DerefMut, Entity, Plugin};
use bevy::utils::hashbrown::HashMap;
use bevy_flurx_ipc::FlurxIpcPlugin;

mod event_emitter;
pub mod handlers;
mod ipc_resolve;
mod load_webview;
mod visible;

#[cfg(debug_assertions)]
mod devtools;
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
))]
mod linux;
mod protocol;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::webview::{
        handlers::prelude::*,
        WryWebViews,
    };
}

pub(crate) struct WebviewPlugin;

impl Plugin for WebviewPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FlurxIpcPlugin>() {
            app.add_plugins(FlurxIpcPlugin);
        }

        app
            .add_plugins((
                LoadWebviewPlugin,
                VisiblePlugin,
                EventEmitterPlugin,
                IpcResolvePlugin,
                WryHandlersPlugin,
                CustomProtocolPlugin,
                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                linux::WebviewSupportLinuxPlugin,
            ))
            .init_non_send_resource::<WryWebViews>();

        #[cfg(debug_assertions)]
        app.add_plugins(devtools::DevtoolsPlugin);
    }
}

/// A hashmap that manages the initialized webview.
///
/// [`World`](bevy::prelude::World) holds this as [`NonSend`](bevy::prelude::NonSend).
#[repr(transparent)]
#[derive(Deref, DerefMut, Default)]
pub struct WryWebViews(pub(crate) HashMap<Entity, wry::WebView>);
