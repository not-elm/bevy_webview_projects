use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Determines whether browser-specific accelerator keys are enabled.
///
/// Default is `true`.
///
/// This setting only works on `Windows`.
#[repr(transparent)]
#[derive(Component, Debug, Eq, PartialEq, Copy, Clone, Reflect, Hash, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct BrowserAcceleratorKeys(pub bool);

impl Default for BrowserAcceleratorKeys {
    fn default() -> Self {
        Self(true)
    }
}
