mod asset;

use crate::webview::protocol::asset::{
    convert_to_response, WryRequestArgs, WryResponseBody, WryResponseHandle, WryResponseLoader,
};
use bevy::app::{App, Plugin};
use bevy::platform::collections::hash_map::HashMap;
use bevy::prelude::*;
use bevy_webview_core::prelude::{Csp, Webview};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use wry::RequestAsyncResponder;

pub struct CustomProtocolPlugin;

impl Plugin for CustomProtocolPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = std::sync::mpsc::channel();
        app.register_type::<WryRequestArgs>()
            .init_asset::<WryResponseBody>()
            .init_asset_loader::<WryResponseLoader>()
            .insert_non_send_resource(WryRequestReceiver(rx))
            .insert_non_send_resource(WryRequestSender(tx))
            .insert_non_send_resource(WryResponseMap(HashMap::default()))
            .add_systems(
                Update,
                (
                    start_load,
                    response.run_if(any_with_component::<WryResponseHandle>),
                    #[cfg(feature = "hot-reload")]
                    hot_reload.run_if(on_event::<AssetEvent<WryResponseBody>>),
                ),
            );
    }
}

#[derive(Default, Component, Deref, DerefMut)]
pub(crate) struct WryResponseHandles(HashMap<AssetId<WryResponseBody>, WryResponseHandle>);

pub struct WryRequest {
    pub webview: Entity,
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
    mut response_handles: Query<&mut WryResponseHandles>,
    rx: NonSend<WryRequestReceiver>,
    asset_server: Res<AssetServer>,
) {
    while let Ok(request) = rx.0.try_recv() {
        let args = WryRequestArgs {
            csp: request.csp,
            path: request.path.clone(),
        };
        let Ok(mut handles) = response_handles.get_mut(request.webview) else {
            continue;
        };
        let response_handle = WryResponseHandle(asset_server.load(request.path.clone()));
        handles.insert(response_handle.0.id(), response_handle.clone());
        commands
            .entity(request.webview)
            .with_child((response_handle, args.clone()));
        map.0.insert(args, request.responder);
    }
}

fn response(
    mut commands: Commands,
    mut map: NonSendMut<WryResponseMap>,
    responses: ResMut<Assets<WryResponseBody>>,
    requests: Query<(Entity, &WryRequestArgs, &WryResponseHandle)>,
) {
    for (request_entity, args, handle) in requests.iter() {
        let Some(response_body) = responses.get(handle.0.id()) else {
            continue;
        };
        let Some(responder) = map.0.remove(args) else {
            continue;
        };
        responder.respond(convert_to_response(response_body.0.clone(), args));
        commands.entity(request_entity).despawn();
    }
}

#[cfg(feature = "hot-reload")]
fn hot_reload(
    mut er: EventReader<AssetEvent<WryResponseBody>>,
    wry_webviews: NonSend<crate::prelude::WryWebViews>,
    webviews: Query<(Entity, &WryResponseHandles), With<Webview>>,
    asset_server: Res<AssetServer>,
) {
    let modified_ids = er
        .read()
        .filter_map(|event| {
            if let AssetEvent::Modified { id } = event {
                Some(id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    for (webview_entity, id) in webviews.iter().filter_map(|(entity, handles)| {
        handles
            .keys()
            .find_map(|id| modified_ids.contains(&id).then_some((entity, id)))
    }) {
        if let Some(webview) = wry_webviews.get(&webview_entity) {
            if let Some(path) = asset_server.get_path(id.untyped()) {
                info!("Reloading webview {webview_entity}: {path:?}");
            }
            if let Err(e) = webview.reload() {
                warn!("Failed to reload webview {webview_entity}: {e}");
            } else {
                info!("Reloaded webview {webview_entity}");
            }
        }
    }
}
