use super::PassedUrl;
use bevy::prelude::{Component, Window};

pub(crate) type BoxedNewWindowRequest =
    Box<dyn Fn(PassedUrl) -> NewWindowResponse + Send + Sync + 'static>;

#[allow(clippy::large_enum_variant)]
/// The response to [`OnNewWindowRequest`].
#[derive(Debug, Clone)]
pub enum NewWindowResponse {
    /// Create new window and open url in it.
    CreateWindow(Window),
    /// Allow open url in current window.
    Allow,
    /// Deny open url.
    Deny,
}

/// Set a new window request handler to decide if incoming url is allowed to be opened.
#[repr(transparent)]
#[derive(Component, Default)]
pub struct OnNewWindowRequest(Option<BoxedNewWindowRequest>);

impl OnNewWindowRequest {
    /// Creates the [`OnNewWindowRequest`].
    pub fn new(f: impl Fn(PassedUrl) -> NewWindowResponse + Send + Sync + 'static) -> Self {
        Self(Some(Box::new(f)))
    }

    /// Take the callback.
    #[inline]
    pub fn take(&mut self) -> Option<BoxedNewWindowRequest> {
        self.0.take()
    }
}
