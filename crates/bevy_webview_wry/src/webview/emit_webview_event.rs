use crate::webview::WryWebViews;
use bevy::prelude::*;
use serde::Serialize;

#[derive(Event)]
pub struct EmitEventToWebview {
    /// The event id
    pub id: String,

    /// The data to be sent to the webview
    pub payload: EventPayload,
}

/// The data to be sent to the webview
pub struct EventPayload(String);

impl EventPayload {
    /// Creates a new [`EventPayload`].
    pub fn new<T: Serialize>(payload: T) -> Self {
        let payload = serde_json::to_string(&payload).expect("Failed to serialize payload");
        Self(payload)
    }
}


pub(crate) struct EventEmitterPlugin;

impl Plugin for EventEmitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(apply_emit_event);
    }
}

fn apply_emit_event(
    trigger: Trigger<EmitEventToWebview>,
    webviews: Query<(Entity, &Name)>,
    wry_webviews: NonSend<WryWebViews>,
) {
    let target = trigger.target();
    // Global event
    if target == Entity::PLACEHOLDER {
        for (entity, _) in webviews.iter() {
            call_javascript_callback(entity, trigger.event(), &webviews, &wry_webviews);
        }
    } else {
        call_javascript_callback(target, trigger.event(), &webviews, &wry_webviews);
    }
}

fn call_javascript_callback(
    webview_entity: Entity,
    event: &EmitEventToWebview,
    webviews: &Query<(Entity, &Name)>,
    wry_webviews: &WryWebViews,
) {
    let Some(webview) = wry_webviews.0.get(&webview_entity) else {
        return;
    };
    let Ok((_, name)) = webviews.get(webview_entity) else {
        return;
    };

    let name = name.as_str();
    let event_id = event.id.as_str();
    let payload = event.payload.0.as_str();
    if let Err(e) = webview.evaluate_script(&format!(
        "window.__FLURX__.__emitEvent('{name}', '{event_id}', {payload});"
    )) {
        error!("{e}");
    }
}
