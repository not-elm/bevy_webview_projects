use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Controls opening and closing the webview devtools.
///
/// This component and [`UseDevtools`](crate::prelude::UseDevtools) must be `true` if you want to open developer tools.
///
/// Default is `false`.
///
/// ## Notes
///
/// * __This only works in debug builds.__
/// * Opening and closing devtools doesn't seem to be supported on some platforms;
#[repr(transparent)]
#[derive(
    Component, Clone, Debug, Eq, PartialEq, Hash, Default, Reflect, Serialize, Deserialize,
)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct IsOpenDevtools(pub bool);
