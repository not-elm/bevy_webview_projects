//! Controls the process of creating the new window : [`wry::WebViewBuilder::with_new_window_req_handler`]

use crate::prelude::{PassedUrl, Webview, WebviewUri};
use crate::webview::handlers::{RegisterWryEvent, WryEvents};
use bevy::prelude::{App, Commands, Entity, Event, EventWriter, Plugin, PreUpdate, Reflect, Res, Window};

/// The event indicating that a new window has been opened.
///
/// [`OnNewWindowRequest`](crate::prelude::OnNewWindowRequest)
#[derive(Event, Clone, Debug, Reflect)]
pub struct NewWindowOpened {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// The entity associated with the new [`Window`].
    pub opened_window_entity: Entity,

    /// The url loaded in new [`Window`].
    pub url: PassedUrl,
}

#[derive(Event, Clone, Debug, Reflect)]
pub(crate) struct NewWindowRequested {
    pub webview_entity: Entity,
    pub url: PassedUrl,
    pub window: Window,
}

pub(crate) struct NewWindowRequestedPlugin;

impl Plugin for NewWindowRequestedPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<NewWindowRequested>()
            .add_event::<NewWindowRequested>()
            .init_resource::<WryEvents<NewWindowRequested>>()
            .register_wry_event::<NewWindowOpened>()
            .add_systems(PreUpdate, open_new_window);
    }
}

fn open_new_window(
    mut commands: Commands,
    mut ew: EventWriter<NewWindowOpened>,
    events: Res<WryEvents<NewWindowRequested>>,
) {
    for request in events.take_events() {
        let opened_window_entity = commands
            .spawn((
                request.window,
                Webview::Uri(WebviewUri(request.url.0.to_string())),
            ))
            .id();

        ew.write(NewWindowOpened {
            webview_entity: request.webview_entity,
            opened_window_entity,
            url: request.url,
        });
    }
}
