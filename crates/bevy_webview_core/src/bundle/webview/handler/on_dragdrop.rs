use bevy::math::IVec2;
use bevy::prelude::{Component, Entity, Event, Reflect};
use std::path::PathBuf;

pub(crate) type BoxedDragDropEventHandler = Box<dyn Fn(DragDropEvent) -> bool + Send + Sync>;

/// Represents the drag-drop events that can be fired by the webview.
#[derive(Event, Clone, Reflect, Debug)]
pub enum DragDropEvent {
    /// [`DragEntered`]
    Enter(DragEntered),
    /// [`DragOver`]
    Over(DragOver),
    /// [`Dropped`]
    Drop(Dropped),
    /// [`DragLeave`]
    Leave(DragLeave),
}

/// Fired when a file or other item is dragged into the Webview.
#[derive(Event, Clone, Reflect, Debug)]
pub struct DragEntered {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// List of paths that are being dragged onto the webview.
    pub paths: Vec<PathBuf>,

    /// Position of the drag operation, relative to the webview top-left corner.
    pub position: IVec2,
}

/// Fired while a file or other item is being dragged over to the web view.
#[derive(Event, Clone, Reflect, Debug)]
pub struct DragOver {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// Position of the drag operation, relative to the webview top-left corner.
    pub position: IVec2,
}

/// Fired when a file or other item is dropped onto the Webview.
#[derive(Event, Clone, Reflect, Debug)]
pub struct Dropped {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// List of paths that are being dropped onto the window.
    pub paths: Vec<PathBuf>,

    /// Position of the drag operation, relative to the webview top-left corner.
    pub position: IVec2,
}

/// Fired when the item under drag left the webview.
#[derive(Event, Copy, Clone, Reflect, Debug)]
pub struct DragLeave {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,
}

/// The handler for drag-drop events sent from the webview.
///
/// If returns `true`, the OS default behavior will be blocked.
#[repr(transparent)]
#[derive(Component, Default)]
pub struct OnDragDrop(Option<BoxedDragDropEventHandler>);

impl OnDragDrop {
    /// No callback is specified.
    ///
    /// The os default behavior of dragdrop does not block it.
    pub const NONE: Self = Self(None);

    /// Create the new [`OnDragDrop`].
    ///
    /// Return `true` in the callback to block the OS' default behavior.
    pub fn new(f: impl Fn(DragDropEvent) -> bool + Send + Sync + 'static) -> Self {
        Self(Some(Box::new(f)))
    }

    /// Take the callback.
    #[inline]
    pub fn take(&mut self) -> Option<BoxedDragDropEventHandler> {
        self.0.take()
    }
}
