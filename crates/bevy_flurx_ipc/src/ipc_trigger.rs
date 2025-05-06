//! Provides a mechanism to convert messages from external processes into [`Trigger`](bevy::prelude::Trigger).

use bevy::prelude::{App, Commands, Entity, Event, IntoScheduleConfigs, Plugin, PreUpdate, Res, ResMut, Resource};
use serde::de::DeserializeOwned;
use std::sync::{Arc, Mutex};

/// The ipc message.
pub struct IpcTriggerMessage {
    /// The target entity of [`Trigger`](bevy::prelude::Trigger).
    pub target: Option<Entity>,

    /// event id
    pub event_id: String,

    /// The serialized main body of the event sent from the webview.
    pub payload: String,
}

/// The structure to send IPC messages.
#[repr(transparent)]
#[derive(Resource, Clone, Default)]
pub struct IpcTriggerSender(Arc<Mutex<Vec<IpcTriggerMessage>>>);

impl IpcTriggerSender {
    /// Push the [`IpcTriggerMessage`] into queue.
    ///
    /// The message pushed will be deserialized into the type registered with [`IpcTriggerExt::add_ipc_trigger`].
    #[inline(always)]
    pub fn send(&self, event: IpcTriggerMessage) {
        self.0.lock().unwrap().push(event);
    }
}

/// Allows you to receive messages from Ipc as [`Trigger`](bevy::prelude::Trigger).
pub trait IpcTriggerExt {
    /// This method allows you to receive messages from Ipc as [`Trigger`](bevy::prelude::Trigger).
    ///
    /// `event_id` is the id that associated with this event.
    fn add_ipc_trigger<Payload>(&mut self, event_id: impl Into<String>) -> &mut Self
    where
        Payload: DeserializeOwned + Event + Send + Sync + 'static;
}

impl IpcTriggerExt for App {
    fn add_ipc_trigger<P>(&mut self, event_id: impl Into<String>) -> &mut Self
    where
        P: DeserializeOwned + Event + Send + Sync + 'static,
    {
        let event_id = event_id.into();
        self.add_event::<P>();
        self.add_systems(PreUpdate, read_receive_ipc_event_from_webview::<P>(event_id).before(cleanup_ipc_trigger_sender));
        self
    }
}

pub(crate) struct IpcTriggerPlugin;

impl Plugin for IpcTriggerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<IpcTriggerSender>()
            .add_systems(PreUpdate, cleanup_ipc_trigger_sender);
    }
}

fn read_receive_ipc_event_from_webview<Payload>(
    event_id: String,
) -> impl Fn(Commands, Res<IpcTriggerSender>)
where
    Payload: DeserializeOwned + Event + Send + Sync + 'static,
{
    move |mut commands: Commands,
          ipc_sender: Res<IpcTriggerSender>,
    | {
        let Ok(messages) = ipc_sender.0.try_lock() else {
            return;
        };
        for message in messages.iter().filter(|m| m.event_id == event_id) {
            let Ok(p1) = serde_json::from_str::<Payload>(&message.payload)  else {
                continue;
            };
            let Ok(p2) = serde_json::from_str::<Payload>(&message.payload)  else {
                continue;
            };
            commands.send_event(p1);
            if let Some(target) = message.target {
                commands.entity(target).trigger(p2);
            } else {
                commands.trigger(p2);
            }
        }
    }
}

fn cleanup_ipc_trigger_sender(
    ipc_sender: ResMut<IpcTriggerSender>,
) {
    if let Ok(mut messages) = ipc_sender.0.try_lock() {
        messages.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::ipc_trigger::{IpcTriggerExt, IpcTriggerSender};
    use crate::prelude::{IpcTriggerMessage, IpcTriggerPlugin};
    use bevy::app::App;
    use bevy::prelude::{Event, ResMut, Trigger};
    use bevy::MinimalPlugins;
    use bevy_test_helper::error::TestResult;
    use bevy_test_helper::resource::bool::{Bool, BoolExtension};
    use bevy_test_helper::resource::DirectResourceControl;
    use bevy_test_helper::BevyTestHelperPlugin;
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Eq, Deserialize, Serialize, Event)]
    struct TestMessage {
        id: String,
    }

    #[test]
    fn test_send_ipc_message() {
        let mut app = trigger_test_app();
        app.add_observer(|_: Trigger<TestMessage>, mut b: ResMut<Bool>| {
            b.set_true();
        });
        app.resource_mut::<IpcTriggerSender>().send(IpcTriggerMessage {
            target: None,
            event_id: "test_message".to_string(),
            payload: serde_json::to_string(&TestMessage {
                id: "test".to_string(),
            }).unwrap(),
        });
        app.update();
        assert!(app.is_bool_true());
    }

    #[test]
    fn test_cleanup_ipc_trigger_sender() -> TestResult {
        let mut app = trigger_test_app();
        app.resource_mut::<IpcTriggerSender>().send(IpcTriggerMessage {
            target: None,
            event_id: "test_message".to_string(),
            payload: serde_json::to_string(&TestMessage {
                id: "test2".to_string(),
            }).unwrap(),
        });
        app.update();
        let sender = app.resource::<IpcTriggerSender>();
        let messages = sender.0.lock().unwrap();
        assert!(messages.is_empty());
        Ok(())
    }

    fn trigger_test_app() -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            BevyTestHelperPlugin,
            IpcTriggerPlugin,
        ));
        app.add_ipc_trigger::<TestMessage>("test_message");
        app
    }
}
