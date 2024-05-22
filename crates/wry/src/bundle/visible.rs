use bevy::prelude::{Component, Plugin, Reflect, ReflectComponent, ReflectDefault};
use serde::{Deserialize, Serialize};


/// Please see [`wry::WebViewBuilder::with_visible`].
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub struct Visible(pub bool);

impl Default for Visible {
    fn default() -> Self {
        Self(true)
    }
}


