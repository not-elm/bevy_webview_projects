use super::PassedUrl;
use bevy::prelude::Component;
use std::path::PathBuf;

pub(crate) type BoxedDownloadHandler =
    Box<dyn FnMut(PassedUrl, &mut PathBuf) -> bool + Send + Sync>;

/// The callback handler for download requests.
///
/// If the return value of the callback is `false`, the download is canceled.
#[repr(transparent)]
#[derive(Component, Default)]
pub struct OnDownload(Option<BoxedDownloadHandler>);

impl OnDownload {
    /// No callback is specified.
    ///
    /// All downloads are allowed and the download destination is not changed.
    pub const NONE: Self = Self(None);

    /// Creates the new [`OnDownload`].
    ///
    /// The first argument of the callback is the URL of the download source,
    /// the second is the destination.
    ///
    /// The download destination is variable and can be changed within the callback.
    ///
    /// The callback returns a `bool` to allow or deny the download.
    pub fn new(f: impl FnMut(PassedUrl, &mut PathBuf) -> bool + Send + Sync + 'static) -> Self {
        Self(Some(Box::new(f)))
    }

    /// Take the callback.
    #[inline]
    pub fn take(&mut self) -> Option<BoxedDownloadHandler> {
        self.0.take()
    }
}
