use bevy::prelude::{
    Color, Component, Reflect, ReflectComponent, ReflectDefault, ReflectDeserialize,
    ReflectSerialize,
};
use serde::{Deserialize, Serialize};

/// Represents the webview background.
///
/// Default is [`Background::Unspecified`].
#[derive(Component, Clone, Debug, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub enum Background {
    /// Default settings will be applied.
    #[default]
    Unspecified,

    /// Sets the specified background color.
    Color(Color),

    /// Sets the webView should be transparent.
    Transparent,
}

impl Background {
    /// Returns `true` if the background is transparent.
    pub const fn is_transparent(&self) -> bool {
        matches!(self, Background::Transparent)
    }
}
