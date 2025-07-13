use crate::bundle::webview::*;
use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use bevy_flurx_ipc::prelude::IpcHandlers;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Represents the display destination of webview.
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_webview_wry::prelude::*;
/// use std::path::PathBuf;
/// use bevy::window::PrimaryWindow;
///
/// App::new()
///     .add_plugins((
///         DefaultPlugins,
///         WebviewWryPlugin{
///             // local root will be flurx://localhost/ui.
///             local_root: PathBuf::from("ui")
///         }
///     ))
///     .run();
///
/// fn spawn_webview(mut commands: Commands, window: Query<Entity, With<PrimaryWindow>>){
///     commands
///         .entity(window.single().expect("Failed to obtain the window"))
///         // The actual URL is flurx://localhost/ui/example.html.
///         // show assets/ui/example.html
///         .insert(Webview::Uri(WebviewUri::relative_local("example.html")));
/// }
/// ```
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[require(
    EnableClipboard,
    AutoPlay,
    BrowserAcceleratorKeys,
    UseDevtools,
    IsOpenDevtools,
    WebviewVisible,
    Background,
    UserAgent,
    Theme,
    InitializeFocused,
    InitializationScripts,
    Incognito,
    HotkeysZoom,
    UseHttpsScheme,
    IpcHandlers,
    OnDownload,
    OnDragDrop,
    OnNavigation,
    OnNewWindowRequest
)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub enum Webview {
    /// Load the webview with the specified remote/local uri.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use bevy_webview_wry::prelude::{Webview, WebviewUri};
    /// Webview::Uri(WebviewUri::new("https://bevyengine.org/"));
    /// Webview::Uri(WebviewUri::relative_local("example.html"));
    /// ```
    Uri(WebviewUri),

    /// Load the webview with the specified html content.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use bevy_webview_wry::prelude::Webview;
    /// Webview::Html("<html><body><h1>Hello world!</h1></body></html>".to_string());
    /// ```
    Html(String),
}

impl Default for Webview {
    fn default() -> Self {
        Webview::Uri(WebviewUri::default())
    }
}

impl From<WebviewUri> for Webview {
    fn from(uri: WebviewUri) -> Self {
        Webview::Uri(uri)
    }
}

/// The Uri that load in the webview.
///
/// If you want to load a local resource, use custom protocol: `flurx://localhost/<ROOT>/<uri>`.
///
/// `<ROOT>` is specified in the plugin field or Resource of the crate that actually implements the Webview.
///
/// Default is `flurx://localhost/`.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub struct WebviewUri(pub String);

impl WebviewUri {
    /// Returns the new [`WebviewUri`].
    pub fn new(uri: impl Into<String>) -> Self {
        Self(uri.into())
    }

    /// Returns the webview uri to load a local resource.
    ///
    /// The url will be in the form of a custom protocol: `flurx://localhost/<ROOT>/<uri>`.
    ///
    /// `<ROOT>` is specified in the plugin field or Resource of the crate that actually implements the Webview.
    pub fn relative_local(uri: impl AsRef<Path>) -> Self {
        let path = PathBuf::from("flurx://localhost/").join(uri.as_ref());
        Self(path.to_string_lossy().to_string())
    }
}

impl Default for WebviewUri {
    fn default() -> Self {
        WebviewUri::relative_local("")
    }
}
