mod asset;

use crate::webview::protocol::asset::{convert_to_response, WryRequestArgs, WryResponseBody, WryResponseHandle, WryResponseLoader};
use bevy::app::{App, Plugin};
use bevy::platform::collections::hash_map::HashMap;
use bevy::prelude::*;
use bevy_webview_core::prelude::Csp;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use wry::RequestAsyncResponder;

pub struct CustomProtocolPlugin;

impl Plugin for CustomProtocolPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = std::sync::mpsc::channel();
        app
            .register_type::<WryRequestArgs>()
            .init_asset::<WryResponseBody>()
            .init_asset_loader::<WryResponseLoader>()
            .insert_non_send_resource(WryRequestReceiver(rx))
            .insert_non_send_resource(WryRequestSender(tx))
            .insert_non_send_resource(WryResponseMap(HashMap::default()))
            .add_systems(Update, (
                start_load,
                response.run_if(any_with_component::<WryResponseHandle>),
            ));
    }
}

pub struct WryRequest {
    pub path: PathBuf,
    pub csp: Option<Csp>,
    pub responder: RequestAsyncResponder,
}

pub struct WryRequestReceiver(pub Receiver<WryRequest>);

#[derive(Clone)]
pub struct WryRequestSender(pub Sender<WryRequest>);

pub struct WryResponseMap(pub HashMap<WryRequestArgs, RequestAsyncResponder>);

fn start_load(
    mut commands: Commands,
    mut map: NonSendMut<WryResponseMap>,
    rx: NonSend<WryRequestReceiver>,
    asset_server: Res<AssetServer>,
) {
    while let Ok(request) = rx.0.try_recv() {
        let args = WryRequestArgs {
            csp: request.csp,
            path: request.path.clone(),
        };
        commands.spawn((
            args.clone(),
            WryResponseHandle(asset_server.load(request.path.clone())),
        ));

        map.0.insert(args, request.responder);
    }
}

fn response(
    mut commands: Commands,
    mut responses: ResMut<Assets<WryResponseBody>>,
    mut handles: Query<(Entity, &WryRequestArgs, &WryResponseHandle)>,
    mut map: NonSendMut<WryResponseMap>,
) {
    for (entity, args, handle) in handles.iter_mut() {
        let Some(response_body) = responses.remove(handle.0.id()) else {
            continue;
        };
        let Some(responder) = map.0.remove(args) else {
            continue;
        };
        responder.respond(convert_to_response(response_body.0, args));
        commands.entity(entity).despawn();
    }
}