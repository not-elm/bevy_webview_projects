use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// `GripZone` specifies the height at which the webview can be gripped
/// by a left-click.
///
/// You can move the webview by dragging it while holding it.
///
///
/// The unit of height is css pixels.
///
/// Default is 20(px).
#[repr(transparent)]
#[derive(
    Debug, Reflect, Copy, Clone, Eq, PartialEq, Hash, Component, Default, Serialize, Deserialize,
)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct GripZone(pub u32);
