use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Set a custom user-agent for the webview.
///
/// Default is not specified.
#[repr(transparent)]
#[derive(
    Component, Debug, Clone, Eq, PartialEq, Hash, Default, Reflect, Serialize, Deserialize,
)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct UserAgent(pub Option<String>);

impl UserAgent {
    /// Creates the new [`UserAgent`].
    pub fn new(user_agent: impl Into<String>) -> Self {
        Self(Some(user_agent.into()))
    }
}
