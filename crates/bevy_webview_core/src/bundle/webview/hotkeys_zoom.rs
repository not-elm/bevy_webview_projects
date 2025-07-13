use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Whether page zooming by hotkeys or gestures is enabled.
///
/// Default is `true`.
#[repr(transparent)]
#[derive(Component, Copy, Clone, Reflect, Eq, PartialEq, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct HotkeysZoom(pub bool);

impl Default for HotkeysZoom {
    fn default() -> Self {
        Self(true)
    }
}
