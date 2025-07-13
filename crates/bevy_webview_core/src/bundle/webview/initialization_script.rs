use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Represents the initialization scripts for the webview.
///
/// A timing when these scripts are executed depends on the webview crates.
#[repr(transparent)]
#[derive(
    Component, Debug, Clone, Eq, PartialEq, Hash, Default, Reflect, Serialize, Deserialize,
)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct InitializationScripts(Vec<String>);

impl InitializationScripts {
    /// Creates the new [`InitializationScripts`].
    pub fn new<S: Into<String>>(scripts: impl IntoIterator<Item = S>) -> Self {
        Self(scripts.into_iter().map(S::into).collect())
    }

    /// Appends the initialization script.
    pub fn append(&mut self, script: impl Into<String>) -> &mut Self {
        self.0.push(script.into());
        self
    }

    /// Converts the initialization scripts to the script code.
    pub fn to_scripts(&self) -> String {
        self.0.join(";")
    }
}
