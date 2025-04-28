//! Defines the ipc commands and the queue to execute them.

use bevy::log::error;
use bevy::platform::collections::HashMap;
use bevy::prelude::{App, Entity, Event, EventWriter, IntoScheduleConfigs, Plugin, PreUpdate, Res, Resource};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

/// The event sent from webview.
///
/// For use this, you need to call the [`IpcEventExt::add_ipc_event`] before app running.
#[derive(Event, Debug, Copy, Clone, Eq, PartialEq)]
pub struct IpcEvent<P> {
    /// The entity associated with webview .
    pub webview_entity: Entity,

    /// The main body of the event sent from the webview.
    pub payload: P,
}

/// The ipc raw event.
///
/// Attach the webview entity to the event info sent from javascript and
/// push itself to [`IpcRawEvents`].
pub struct IpcRawEvent {
    /// The entity associated with webview .
    pub webview_entity: Entity,

    /// The main body of the event sent from the webview.
    pub body: IpcRawEventBody,
}

/// The ipc event info expected to be sent from javascript.
#[derive(Deserialize)]
pub struct IpcRawEventBody {
    /// event id
    pub event_id: String,

    /// The serialized main body of the event sent from the webview.
    pub payload: String,
}

/// The ipc commands that exists only one in the [`World`](bevy::prelude::World).
#[repr(transparent)]
#[derive(Resource, Clone, Default)]
pub struct IpcRawEvents(Arc<Mutex<Vec<IpcRawEvent>>>);

impl IpcRawEvents {
    /// Push the [`IpcRawEvent`] into queue.
    ///
    /// The pushed event will be deserialized, and then will send as [`IpcEvent`].
    #[inline(always)]
    pub fn push(&self, event: IpcRawEvent) {
        self.0.lock().unwrap().push(event);
    }
}

type DeserializeFn = Box<dyn Fn(IpcRawEvent) + Send + Sync>;

#[repr(transparent)]
#[derive(Resource, Default)]
struct IpcEventHandlers(Arc<Mutex<HashMap<String, DeserializeFn>>>);

#[repr(transparent)]
#[derive(Resource)]
struct IpcEvents<P>(Arc<Mutex<Vec<IpcEvent<P>>>>);

/// Add an [`IpcEvent`] into [`App`].
pub trait IpcEventExt {
    /// This method registers [`IpcEvent<Payload>`](IpcEvent), which can be read just like a normal bevy event.
    ///
    /// `event_id` is the id that associated with this event.
    ///
    /// From javascript side, you can emit the event as follows:
    /// `window.__FLURX__.emit(<event_id>, payload)`
    fn add_ipc_event<Payload>(&mut self, event_id: impl Into<String>) -> &mut Self
    where
        Payload: DeserializeOwned + Send + Sync + 'static;
}

impl IpcEventExt for App {
    fn add_ipc_event<P>(&mut self, event_id: impl Into<String>) -> &mut Self
    where
        P: DeserializeOwned + Send + Sync + 'static,
    {
        let events = IpcEvents::<P>(Arc::new(Mutex::new(vec![])));
        self.add_event::<IpcEvent<P>>()
            .insert_resource(IpcEvents(Arc::clone(&events.0)))
            .add_systems(PreUpdate, send_ipc_events::<P>.after(read_raw_events));

        let handlers = self
            .world_mut()
            .get_resource_or_insert_with::<IpcEventHandlers>(IpcEventHandlers::default);
        let event_id = event_id.into();
        handlers.0.lock().unwrap().insert(
            event_id.clone(),
            Box::new(
                move |raw_event| match serde_json::from_str::<P>(&raw_event.body.payload) {
                    Ok(payload) => {
                        events.0.lock().unwrap().push(IpcEvent {
                            webview_entity: raw_event.webview_entity,
                            payload,
                        });
                    }
                    Err(e) => {
                        error!("Failed ipc event deserialize event_id={event_id}: {e}");
                    }
                },
            ),
        );
        self
    }
}

pub(crate) struct FlurxIpcEventPlugin;

impl Plugin for FlurxIpcEventPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IpcRawEvents>()
            .init_resource::<IpcEventHandlers>()
            .add_systems(PreUpdate, read_raw_events);
    }
}

fn send_ipc_events<E: DeserializeOwned + Send + Sync + 'static>(
    mut ew: EventWriter<IpcEvent<E>>,
    ipc_events: Res<IpcEvents<E>>,
) {
    if let Ok(mut guard) = ipc_events.0.try_lock() {
        ew.write_batch(std::mem::take(&mut *guard));
    }
}

fn read_raw_events(ipc_raw_events: Res<IpcRawEvents>, ipc_event_handlers: Res<IpcEventHandlers>) {
    let Ok(mut raw_events) = ipc_raw_events.0.try_lock() else {
        return;
    };
    let events = std::mem::take(&mut *raw_events);
    if events.is_empty() {
        return;
    }
    let Ok(handlers) = ipc_event_handlers.0.lock() else {
        raw_events.extend(events);
        return;
    };
    drop(raw_events);

    for raw_event in events {
        if let Some(handler) = handlers.get(&raw_event.body.event_id) {
            handler(raw_event);
        }
    }
}
