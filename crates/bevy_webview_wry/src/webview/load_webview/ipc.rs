use bevy::ecs::system::SystemParam;
use bevy::log::error;
use bevy::prelude::{Entity, Res};
use bevy_flurx_ipc::ipc_commands::{IpcCommand, IpcCommands, Payload};
use bevy_flurx_ipc::prelude::{IpcTriggerMessage, IpcTriggerSender};
use serde::Deserialize;
use wry::WebViewBuilder;

#[derive(SystemParam)]
pub(crate) struct IpcHandlerParams<'w> {
    ipc_commands: Res<'w, IpcCommands>,
    ipc_raw_events: Res<'w, IpcTriggerSender>,
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "message")]
enum WebviewIpcMessage {
    Command(Payload),
    Event(IpcTriggerBody),
}

#[derive(Deserialize)]
struct IpcTriggerBody {
    /// event id
    pub event_id: String,
    /// The serialized main body of the event sent from the webview.
    pub payload: String,
}

impl IpcHandlerParams<'_> {
    pub(crate) fn feed_ipc<'a>(
        &self,
        webview_entity: Entity,
        builder: WebViewBuilder<'a>,
    ) -> WebViewBuilder<'a> {
        let ipc_commands = self.ipc_commands.clone();
        let sender = self.ipc_raw_events.clone();

        builder.with_ipc_handler(move |request| {
            match serde_json::from_str::<WebviewIpcMessage>(request.body()) {
                Ok(WebviewIpcMessage::Command(payload)) => {
                    ipc_commands.push(IpcCommand {
                        entity: webview_entity,
                        payload,
                    });
                }
                Ok(WebviewIpcMessage::Event(body)) => {
                    sender.send(IpcTriggerMessage {
                        target: Some(webview_entity),
                        event_id: body.event_id,
                        payload: body.payload,
                    });
                }
                Err(e) => {
                    error!("failed deserialize bevy_flurx_ipc message: {e}");
                }
            }
        })
    }
}
