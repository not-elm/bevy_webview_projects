//! Declares the webview components.

pub use auto_play::AutoPlay;
pub use background::Background;
use bevy::prelude::Bundle;
use bevy_flurx_ipc::component::IpcHandlers;
pub use browser_accelerator_keys::BrowserAcceleratorKeys;
pub use csp::Csp;
pub use enable_clipboard::EnableClipboard;
pub use focused::InitializeFocused;
pub use handler::*;
pub use hotkeys_zoom::HotkeysZoom;
pub use https_scheme::UseHttpsScheme;
pub use incognito::Incognito;
pub use initialization_script::InitializationScripts;
pub use is_open_devtools::IsOpenDevtools;
pub use theme::Theme;
pub use use_devtools::UseDevtools;
pub use user_agent::UserAgent;
pub use visible::WebviewVisible;
pub use webview_uri::*;

mod auto_play;
mod background;
mod browser_accelerator_keys;
mod enable_clipboard;
mod focused;
mod handler;
mod hotkeys_zoom;
mod https_scheme;
mod incognito;
mod is_open_devtools;
mod theme;
mod use_devtools;
mod user_agent;
mod visible;
mod webview_uri;
mod csp;
mod initialization_script;

/// The following is a list of required components for generating a webview.
///
/// All components defined in this embedding are registered as required components in [`Webview`].
#[derive(Bundle, Default)]
pub struct WebViewBundle {
    /// [`Webview`]
    pub webview: Webview,

    /// [`AutoPlay`]
    pub auto_play: AutoPlay,

    /// [`BrowserAcceleratorKeys`]
    pub browser_accelerator_keys: BrowserAcceleratorKeys,

    /// [`EnableClipboard`]
    pub enable_clipboard: EnableClipboard,

    /// [`UseDevtools`]
    pub use_devtools: UseDevtools,

    /// [`IsOpenDevtools`]
    pub is_open_devtools: IsOpenDevtools,

    /// [`WebviewVisible`]
    pub visible: WebviewVisible,

    /// [`Background`]
    pub background: Background,

    /// [`UserAgent`]
    pub user_agent: UserAgent,

    /// [`Theme`]
    pub theme: Theme,

    /// [`InitializeFocused`]
    pub initialize_focused: InitializeFocused,

    /// [`Incognito`]
    pub incognito: Incognito,

    /// [`HotkeysZoom`]
    pub hotkeys_zoom: HotkeysZoom,

    /// [`UseHttpsScheme`]
    pub use_https_scheme: UseHttpsScheme,

    /// [`IpcHandlers`]
    pub ipc_handlers: IpcHandlers,

    /// [`OnDownload`]
    pub on_download: OnDownload,

    /// [`OnDragDrop`]
    pub on_dragdrop: OnDragDrop,

    /// [`OnNavigation`]
    pub on_navigation: OnNavigation,

    /// [`OnNewWindowRequest`]
    pub on_new_window_request: OnNewWindowRequest,
}
