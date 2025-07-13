//! Controls dragdrop events.

use crate::webview::handlers::WryEvents;
use bevy::prelude::{App, EventWriter, Plugin, PreUpdate, Res};
use bevy_webview_core::prelude::{DragDropEvent, DragEntered, DragLeave, DragOver, Dropped};

pub(crate) struct WryDragDrop {
    pub event: DragDropEvent,
}

pub(crate) struct DragDropPlugin;

impl Plugin for DragDropPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WryEvents<WryDragDrop>>()
            .add_systems(PreUpdate, send_dragdrop_event);
    }
}

fn send_dragdrop_event(
    mut entered: EventWriter<DragEntered>,
    mut over: EventWriter<DragOver>,
    mut dropped: EventWriter<Dropped>,
    mut leaved: EventWriter<DragLeave>,
    events: Res<WryEvents<WryDragDrop>>,
) {
    for event in events.take_events() {
        match event.event {
            DragDropEvent::Enter(enter) => {
                entered.write(enter);
            }
            DragDropEvent::Over(event) => {
                over.write(event);
            }
            DragDropEvent::Drop(event) => {
                dropped.write(event);
            }
            DragDropEvent::Leave(event) => {
                leaved.write(event);
            }
        }
    }
}
