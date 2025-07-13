use super::PassedUrl;
use bevy::prelude::Component;

pub(crate) type BoxedNavigateHandler = Box<dyn Fn(PassedUrl) -> bool + Send + Sync + 'static>;

/// Set a navigation handler to decide if incoming url is allowed to navigate.
#[repr(transparent)]
#[derive(Component, Default)]
pub struct OnNavigation(Option<BoxedNavigateHandler>);

impl OnNavigation {
    /// No callback is specified.
    ///
    /// All navigation is permitted.
    pub const NONE: Self = Self(None);

    /// Creates the new [`OnNavigation`].
    ///
    /// If the return value of the callback is `false`, navigation is canceled.
    pub fn new(f: impl Fn(PassedUrl) -> bool + Send + Sync + 'static) -> Self {
        Self(Some(Box::new(f)))
    }

    /// Take the callback.
    pub fn take(&mut self) -> Option<BoxedNavigateHandler> {
        self.0.take()
    }
}
