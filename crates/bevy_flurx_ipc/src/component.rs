//! Defines a handler that executes the [`Action`](bevy_flurx::prelude::Action) of the command.

use crate::ipc_commands::IpcCommand;
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    Commands, Component, Entity, Reflect, ReflectComponent, ReflectDeserialize, ReflectSerialize,
};
use serde::{Deserialize, Serialize};

/// The ipc invoke handlers.
#[repr(transparent)]
#[derive(Component, Default)]
pub struct IpcHandlers(pub(crate) HashMap<String, IpcHandler>);

impl IpcHandlers {
    /// Create a new [`IpcHandlers`].
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use bevy_flurx::prelude::*;
    /// use bevy_flurx_ipc::prelude::*;
    ///
    /// #[command]
    /// fn hello() -> ActionSeed<(), String>{
    ///     once::run(||{
    ///         "hello world".to_string()
    ///     })
    /// }
    ///
    /// IpcHandlers::new([hello]);
    /// ```
    pub fn new<H>(handlers: impl IntoIterator<Item = H>) -> Self
    where
        H: Into<IpcHandler>,
    {
        let mut me = Self::default();
        for handler in handlers.into_iter() {
            me.register(handler);
        }
        me
    }

    /// Add a [`IpcHandler`].
    pub fn register(&mut self, handler: impl Into<IpcHandler>) {
        let handler = handler.into();
        self.0.insert(handler.id.clone(), handler);
    }

    /// Returns the function that creates the future if exists related to `id`.
    #[inline]
    pub fn get(&self, id: &str) -> Option<IpcFn> {
        self.0.get(id).map(|handler| handler.f)
    }
}

impl From<Vec<IpcHandler>> for IpcHandlers {
    fn from(value: Vec<IpcHandler>) -> Self {
        let mut handlers = Self::default();
        for handler in value {
            handlers.register(handler);
        }
        handlers
    }
}

/// The function created by command macro.
pub type IpcFn = fn(commands: &mut Commands, IpcCommand);

/// The ipc invoke handler.
///
/// Usually created via [`command`](bevy_flurx_ipc_macro::command).
pub struct IpcHandler {
    id: String,
    f: IpcFn,
}

impl IpcHandler {
    /// Create a new handler.
    ///
    /// The `id` is used when invoking from javascript.
    pub fn new(id: impl Into<String>, f: IpcFn) -> Self {
        Self { id: id.into(), f }
    }

    /// Returns the ipc-id.
    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<F> From<F> for IpcHandler
where
    F: Fn() -> IpcHandler,
{
    fn from(f: F) -> Self {
        f()
    }
}

/// This is one of the optional arguments passed to the ipc command.
///
/// It represents the entity associated with the `Webview` components
/// such as [`IpcHandler`].
#[repr(transparent)]
#[derive(Component, Copy, Clone, Reflect, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct WebviewEntity(pub Entity);
