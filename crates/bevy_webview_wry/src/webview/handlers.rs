//! Controls `wry` event handlers.

use crate::prelude::{
    NewWindowResponse, OnDownload, OnDragDrop, OnNavigation, OnNewWindowRequest, PassedUrl,
};
use crate::webview::handlers::document_title_changed::{DocumentTitleChanged, DocumentTitlePlugin};
use crate::webview::handlers::download::{DownloadCompleted, DownloadPlugin, DownloadStarted};
use crate::webview::handlers::dragdrop::{DragDropPlugin, WryDragDrop};
use crate::webview::handlers::navigation::{Navigated, NavigationPlugin};
use crate::webview::handlers::new_window_request::{NewWindowRequested, NewWindowRequestedPlugin};
use crate::webview::handlers::page_load::{PageLoadFinished, PageLoadPlugin, PageLoadStarted};
use bevy::ecs::system::SystemParam;
use bevy::math::IVec2;
use bevy::prelude::{App, Entity, Event, EventWriter, Mut, Plugin, PreUpdate, Res, Resource};
use bevy::reflect::GetTypeRegistration;
use bevy_webview_core::prelude::{DragDropEvent, DragEntered, DragLeave, DragOver, Dropped};
use std::sync::{Arc, Mutex};
use wry::{PageLoadEvent, WebViewBuilder};

pub mod document_title_changed;
pub mod download;
pub mod dragdrop;
pub mod navigation;
pub mod new_window_request;
pub mod page_load;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::webview::handlers::{
        document_title_changed::DocumentTitleChanged,
        download::{DownloadCompleted, DownloadStarted},
        navigation::Navigated,
        new_window_request::*,
        page_load::{PageLoadFinished, PageLoadStarted},
    };
}

pub(crate) struct WryHandlersPlugin;

impl Plugin for WryHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DocumentTitlePlugin,
            DragDropPlugin,
            PageLoadPlugin,
            NavigationPlugin,
            DownloadPlugin,
            NewWindowRequestedPlugin,
        ));
    }
}

#[derive(Resource)]
pub(crate) struct WryEvents<T>(pub Arc<Mutex<Vec<T>>>);

impl<T> WryEvents<T> {
    #[inline]
    pub fn push(&self, t: T) {
        self.0.lock().unwrap().push(t);
    }

    #[inline]
    pub fn take_events(&self) -> Vec<T> {
        self.0
            .try_lock()
            .map(|mut guard| std::mem::take(&mut *guard))
            .unwrap_or_default()
    }
}

impl<T> Default for WryEvents<T> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }
}

impl<T> Clone for WryEvents<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

trait RegisterWryEvent {
    fn register_wry_event<E: Event + GetTypeRegistration>(&mut self) -> &mut Self;
}

impl RegisterWryEvent for App {
    fn register_wry_event<E: Event + GetTypeRegistration>(&mut self) -> &mut Self {
        self.register_type::<E>()
            .add_event::<E>()
            .init_resource::<WryEvents<E>>()
            .add_systems(PreUpdate, send_wry_events::<E>)
    }
}

pub(crate) type HandlerQueries<'a> = (
    &'a mut OnDownload,
    &'a mut OnDragDrop,
    &'a mut OnNavigation,
    &'a mut OnNewWindowRequest,
);

type HandlerQueryArgs<'a> = (
    Mut<'a, OnDownload>,
    Mut<'a, OnDragDrop>,
    Mut<'a, OnNavigation>,
    Mut<'a, OnNewWindowRequest>,
);

#[derive(SystemParam)]
pub(crate) struct WryEventParams<'w> {
    page_load_started_events: Res<'w, WryEvents<PageLoadStarted>>,
    page_load_finished_events: Res<'w, WryEvents<PageLoadFinished>>,
    document_title_events: Res<'w, WryEvents<DocumentTitleChanged>>,
    drag_drop_events: Res<'w, WryEvents<WryDragDrop>>,
    navigation_events: Res<'w, WryEvents<Navigated>>,
    download_started_events: Res<'w, WryEvents<DownloadStarted>>,
    download_completed_events: Res<'w, WryEvents<DownloadCompleted>>,
    new_win_req_events: Res<'w, WryEvents<NewWindowRequested>>,
}

impl WryEventParams<'_> {
    pub(crate) fn feed_handlers<'a>(
        &self,
        webview_entity: Entity,
        (
            mut on_download,
            mut on_dragdrop,
            mut on_navigation,
            mut on_new_window_request
        ): HandlerQueryArgs,
        builder: WebViewBuilder<'a>,
    ) -> WebViewBuilder<'a> {
        let builder = self.feed_page_load(webview_entity, builder);
        let builder = self.feed_document_title_changed(webview_entity, builder);
        let builder = self.feed_dragdrop(webview_entity, builder, &mut on_dragdrop);
        let builder = self.feed_navigation(webview_entity, builder, &mut on_navigation);
        let builder = self.feed_download(webview_entity, builder, &mut on_download);
        self.feed_new_window_request(webview_entity, builder, &mut on_new_window_request)
    }

    fn feed_page_load<'a>(
        &self,
        webview_entity: Entity,
        builder: WebViewBuilder<'a>,
    ) -> WebViewBuilder<'a> {
        let started_events = self.page_load_started_events.clone();
        let finished_events = self.page_load_finished_events.clone();
        builder.with_on_page_load_handler(move |event, url| {
            let url = PassedUrl(url);
            match event {
                PageLoadEvent::Started => {
                    started_events.push(PageLoadStarted {
                        webview_entity,
                        url,
                    });
                }
                PageLoadEvent::Finished => {
                    finished_events.push(PageLoadFinished {
                        webview_entity,
                        url,
                    });
                }
            }
        })
    }

    fn feed_document_title_changed<'a>(
        &self,
        webview_entity: Entity,
        builder: WebViewBuilder<'a>,
    ) -> WebViewBuilder<'a> {
        let events = self.document_title_events.clone();
        builder.with_document_title_changed_handler(move |document_title: String| {
            events.push(DocumentTitleChanged {
                document_title,
                webview_entity,
            });
        })
    }

    fn feed_dragdrop<'a>(
        &self,
        webview_entity: Entity,
        builder: WebViewBuilder<'a>,
        on_dragdrop: &mut OnDragDrop,
    ) -> WebViewBuilder<'a> {
        let events = self.drag_drop_events.clone();
        let on_dragdrop = on_dragdrop.take().unwrap_or(Box::new(|_| false));

        builder.with_drag_drop_handler(move |event| {
            let Some(event) = convert_drag_drop_event(webview_entity, event) else {
                return true;
            };
            events.push(WryDragDrop {
                event: event.clone(),
            });
            on_dragdrop(event)
        })
    }

    fn feed_navigation<'a>(
        &self,
        webview_entity: Entity,
        builder: WebViewBuilder<'a>,
        on_navigation: &mut OnNavigation,
    ) -> WebViewBuilder<'a> {
        let on_navigation = on_navigation.take().unwrap_or(Box::new(|_| true));

        let events = self.navigation_events.clone();
        builder.with_navigation_handler(move |uri| {
            // FIXME: Not sure why, but sending ipc-command sometimes sends this uri.
            if uri == "flurx://localhost/?" {
                return false;
            }

            let uri = PassedUrl(uri);
            let allow_navigation = on_navigation(uri.clone());
            if allow_navigation {
                events.push(Navigated {
                    webview_entity,
                    uri,
                });
            }
            allow_navigation
        })
    }

    fn feed_download<'a>(
        &self,
        webview_entity: Entity,
        builder: WebViewBuilder<'a>,
        on_download: &mut OnDownload,
    ) -> WebViewBuilder<'a> {
        let mut on_download = on_download.take().unwrap_or(Box::new(|_, _| true));

        let started = self.download_started_events.clone();
        let finished = self.download_completed_events.clone();
        builder
            .with_download_started_handler(move |source_url, dest| {
                let source_url = PassedUrl(source_url);
                if on_download(source_url.clone(), dest) {
                    started.push(DownloadStarted {
                        webview_entity,
                        source_url,
                        dest: dest.clone(),
                    });
                    true
                } else {
                    false
                }
            })
            .with_download_completed_handler(move |source_url, dest, succeed| {
                finished.push(DownloadCompleted {
                    webview_entity,
                    source_url: PassedUrl(source_url),
                    dest,
                    succeed,
                });
            })
    }

    fn feed_new_window_request<'a>(
        &self,
        webview_entity: Entity,
        builder: WebViewBuilder<'a>,
        on_new_window_request: &mut OnNewWindowRequest,
    ) -> WebViewBuilder<'a> {
        let events = self.new_win_req_events.clone();
        let on_new_window_request = on_new_window_request
            .take()
            .unwrap_or(Box::new(|_| NewWindowResponse::Allow));

        builder.with_new_window_req_handler(move |url| {
            let url = PassedUrl(url);
            match on_new_window_request(url.clone()) {
                NewWindowResponse::CreateWindow(window) => {
                    events.push(NewWindowRequested {
                        webview_entity,
                        url,
                        window,
                    });
                    false
                }
                NewWindowResponse::Allow => true,
                NewWindowResponse::Deny => false,
            }
        })
    }
}

fn convert_drag_drop_event(
    webview_entity: Entity,
    event: wry::DragDropEvent,
) -> Option<DragDropEvent> {
    match event {
        wry::DragDropEvent::Enter { paths, position } => Some(DragDropEvent::Enter(DragEntered {
            webview_entity,
            paths,
            position: IVec2::new(position.0, position.1),
        })),
        wry::DragDropEvent::Over { position } => Some(DragDropEvent::Over(DragOver {
            webview_entity,
            position: IVec2::new(position.0, position.1),
        })),
        wry::DragDropEvent::Drop { paths, position } => Some(DragDropEvent::Drop(Dropped {
            webview_entity,
            paths,
            position: IVec2::new(position.0, position.1),
        })),
        wry::DragDropEvent::Leave => Some(DragDropEvent::Leave(DragLeave { webview_entity })),
        _ => None,
    }
}

#[inline]
fn send_wry_events<E: Event>(mut ew: EventWriter<E>, events: Res<WryEvents<E>>) {
    ew.write_batch(events.take_events());
}
