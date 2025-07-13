use bevy::prelude::Reflect;
pub use on_download::*;
pub use on_dragdrop::*;
pub use on_navigation::*;
pub use on_new_window_request::*;

mod on_download;
mod on_dragdrop;
mod on_navigation;
mod on_new_window_request;

/// Represents the url that is passed to an event handlers such as [`OnNewWindowRequest`].
///
/// Note that the local url will be a custom protocol compliant URL.
///
/// - macOS, iOS and Linux: `flurx://<path>`
/// - Windows and Android: `http(s)://flurx.<path>`
#[repr(transparent)]
#[derive(Debug, Clone, Eq, PartialEq, Reflect)]
pub struct PassedUrl(pub String);
