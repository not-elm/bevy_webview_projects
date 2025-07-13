use crate::prelude::{
    AutoPlay, Background, Bounds, DragDropEvent, DragEntered, DragLeave, DragOver, Dropped,
    EmbedWithin, EnableClipboard, GripZone, HotkeysZoom, Incognito, InitializeFocused,
    IsOpenDevtools, PassedUrl, Resizable, Theme, UseDevtools, UseHttpsScheme, WebviewUri,
    WebviewVisible,
};
use bevy::prelude::{
    App, Component, Plugin, Reflect, ReflectComponent, ReflectDeserialize, ReflectSerialize,
};
use serde::{Deserialize, Serialize};

pub mod embedding;
pub mod webview;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::bundle::{WebviewInitialized, embedding::*, webview::*};
}

/// Marker component indicating that the webview has been initialized.
#[derive(Component, Reflect, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct WebviewInitialized(pub ());

/// Register all core bundles to the [`AppTypeRegistry`](bevy::prelude::AppTypeRegistry).
pub struct WebViewBundlesPlugin;

impl Plugin for WebViewBundlesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WebviewInitialized>()
            .register_type::<Bounds>()
            .register_type::<GripZone>()
            .register_type::<Resizable>()
            .register_type::<EmbedWithin>()
            .register_type::<AutoPlay>()
            .register_type::<Background>()
            .register_type::<EnableClipboard>()
            .register_type::<WebviewUri>()
            .register_type::<UseDevtools>()
            .register_type::<IsOpenDevtools>()
            .register_type::<WebviewVisible>()
            .register_type::<Theme>()
            .register_type::<InitializeFocused>()
            .register_type::<HotkeysZoom>()
            .register_type::<Incognito>()
            .register_type::<UseHttpsScheme>()
            .register_type::<PassedUrl>()
            .register_type::<DragDropEvent>()
            .register_type::<DragEntered>()
            .register_type::<DragOver>()
            .register_type::<DragLeave>()
            .register_type::<Dropped>()
            .register_type::<EmbedWithin>()
            .register_type::<Bounds>()
            .register_type::<Resizable>()
            .add_event::<DragDropEvent>()
            .add_event::<DragEntered>()
            .add_event::<DragOver>()
            .add_event::<DragLeave>()
            .add_event::<Dropped>();
    }
}
