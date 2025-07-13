//! Provides the mechanism to spawn a webview as a child of an existing window using [`wry::WebViewBuilder::build_as_child`].

use bevy::prelude::{App, Plugin};
use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, Vec2};
use grip_zone::GripZonePlugin;
use resize::ResizePlugin;
use serde::{Deserialize, Serialize};

mod grip_zone;
mod resize;

#[allow(missing_docs)]
pub mod prelude {
    pub use bevy_webview_core::bundle::embedding::*;
}

/// This is a webview that uses [`wry::WebViewBuilder::new_as_child`] to spawn a webview as a child of an existing window.
pub(crate) struct EmbeddingWebviewPlugin;

impl Plugin for EmbeddingWebviewPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CurrentMoving>()
            .add_plugins((ResizePlugin, GripZonePlugin));
    }
}

#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
struct CurrentMoving(pub Vec2);
