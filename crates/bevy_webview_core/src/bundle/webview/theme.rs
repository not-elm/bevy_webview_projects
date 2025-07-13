use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Represents a webview theme.
///
/// Default is [`Theme::Auto`].
///
/// This setting only works on `Windows`.
#[derive(Default, Component, Copy, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    /// System theme
    #[default]
    Auto,

    /// Light theme
    Light,

    /// Dark theme
    Dark,
}
