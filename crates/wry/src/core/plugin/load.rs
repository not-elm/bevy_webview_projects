use bevy::app::{App, Plugin, PreUpdate, Update};
use bevy::prelude::{Commands, Entity, In, NonSend, NonSendMut, Or, Query, Res, Window, With, Without};
use bevy::winit::WinitWindows;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Reactor;
use wry::{WebView, WebViewBuilder, WebViewBuilderExtWindows};

use bevy_flurx_ipc::ipc_commands::{IpcCommand, IpcCommands};

use crate::as_child::bundle::{Bounds, ParentWindow};
use crate::core::bundle::{AutoPlay, Background, BrowserAcceleratorKeys, EnableClipboard, HotkeysZoom, HttpsScheme, Incognito, InitializeFocused, Theme, Uri, UseDevtools, UserAgent, Visible};
use crate::core::plugin::load::protocol::set_protocol;
use crate::core::plugin::on_page_load::{OnPageArgs, PageLoadEventQueue};
use crate::core::plugin::WebviewMap;
use crate::core::WebviewInitialized;
use crate::prelude::Toolbar;

mod protocol;

pub struct LoadWebviewPlugin;

impl Plugin for LoadWebviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, setup_new_windows);
    }
}


type Configs1<'a> = (
    &'a UseDevtools,
    &'a AutoPlay,
    &'a EnableClipboard,
    &'a Visible,
    &'a Background,
    &'a Theme,
    &'a Incognito,
    &'a BrowserAcceleratorKeys,
    &'a HttpsScheme
);

type Configs2<'a> = (
    &'a InitializeFocused,
    &'a HotkeysZoom,
    &'a UserAgent,
    &'a Uri,
    Option<&'a Toolbar>
);

fn setup_new_windows(
    mut commands: Commands,
    views: Query<(
        Entity,
        Configs1,
        Configs2,
        Option<&ParentWindow>,
        Option<&Bounds>
    ), (Without<WebviewInitialized>, Or<(With<Window>, With<ParentWindow>)>)>,
    ipc_commands: Res<IpcCommands>,
    load_queue: Res<PageLoadEventQueue>,
    windows: NonSend<WinitWindows>,
) {
    for (
        entity,
        configs1,
        configs2,
        parent_window,
        bounds
    ) in views.iter() {
        let builder = {
            let Some(builder) = new_builder(entity, &parent_window, &bounds, &windows) else {
                continue;
            };
            let ipc_commands = ipc_commands.clone();
            let load_queue = load_queue.0.clone();
            builder
                .with_on_page_load_handler(move |event, uri| {
                    load_queue.lock().unwrap().push(OnPageArgs {
                        event,
                        uri,
                        entity,
                    });
                })
                .with_ipc_handler(move |request| {
                    ipc_commands.push(IpcCommand {
                        entity,
                        payload: serde_json::from_str(request.body()).unwrap(),
                    });
                })
        };
        let builder = feed_configs1(builder, configs1);
        let builder = feed_configs2(builder, configs2);

        let webview = builder.build().unwrap();
        if let Some(bounds) = bounds {
            // For some reason, `WebViewBuilder::with_bounds` alone doesn't render
            webview.set_bounds(bounds.as_wry_rect()).unwrap();
        }

        commands.entity(entity).insert(WebviewInitialized(()));
        commands.spawn(Reactor::schedule(move |task| async move {
            task.will(Update, once::run(insert_webview).with((entity, webview))).await;
        }));
    }
}

fn new_builder<'a>(
    entity: Entity,
    parent_window: &Option<&ParentWindow>,
    bounds: &Option<&Bounds>,
    windows: &'a WinitWindows,
) -> Option<WebViewBuilder<'a>> {
    if let Some(ParentWindow(parent_entity)) = parent_window {
        let mut builder = WebViewBuilder::new_as_child(windows.get_window(*parent_entity)?);
        if let Some(bounds) = bounds {
            builder = builder.with_bounds(bounds.as_wry_rect());
        }
        Some(builder)
    } else {
        Some(WebViewBuilder::new(windows.get_window(entity)?))
    }
}

fn feed_configs1<'a>(
    builder: WebViewBuilder<'a>,
    (
        dev_tools,
        auto_play,
        enable_clipboard,
        visible,
        background,
        theme,
        incognito,
        browser_accelerator_keys,
        https_scheme
    ): Configs1,
) -> WebViewBuilder<'a> {
    let builder = builder
        .with_devtools(dev_tools.0)
        .with_autoplay(auto_play.0)
        .with_clipboard(enable_clipboard.0)
        .with_visible(visible.0)
        .with_theme(theme.as_wry_theme())
        .with_incognito(incognito.0)
        .with_browser_accelerator_keys(browser_accelerator_keys.0)
        .with_https_scheme(https_scheme.0);

    match background {
        Background::Unspecified => builder,
        Background::Transparent => builder.with_transparent(true),
        Background::Color(color) => {
            let rgba = color.as_rgba_u8();
            builder.with_background_color((rgba[0], rgba[1], rgba[2], rgba[3]))
        }
    }
}

fn feed_configs2<'a>(
    builder: WebViewBuilder<'a>,
    (
        focused,
        hotkeys_zoom,
        user_agent,
        uri,
        toolbar
    ): Configs2,
) -> WebViewBuilder<'a> {
    let mut builder = builder
        .with_focused(focused.0)
        .with_hotkeys_zoom(hotkeys_zoom.0)
        .with_initialization_script(&format!(
            "{}{}{}",
            include_str!("../../../scripts/api.js"),
            include_str!("../../../scripts/mouse.js"),
            toolbar.and_then(|toolbar| toolbar.script()).unwrap_or_default()
        ));

    if let Some(user_agent) = user_agent.0.as_ref() {
        builder = builder.with_user_agent(user_agent);
    }

    set_protocol(builder, uri)
}

fn insert_webview(
    In((entity, webview)): In<(Entity, WebView)>,
    mut view_map: NonSendMut<WebviewMap>,
) {
    view_map.0.insert(entity, webview);
}