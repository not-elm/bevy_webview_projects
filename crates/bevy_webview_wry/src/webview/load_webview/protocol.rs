use bevy::log::error;
use wry::WebViewBuilder;

use crate::WryLocalRoot;
use crate::prelude::{Csp, Webview};
use crate::webview::protocol::{WryRequest, WryRequestSender};

pub fn feed_uri<'a>(
    builder: WebViewBuilder<'a>,
    webview: &Webview,
    local_root: &WryLocalRoot,
    csp: Option<Csp>,
    request_sender: WryRequestSender,
) -> WebViewBuilder<'a> {
    let builder = match webview {
        Webview::Uri(uri) => builder.with_url(&uri.0),
        Webview::Html(html) => builder.with_html(html),
    };
    feed_custom_protocol(builder, local_root.clone(), csp, request_sender)
}

fn feed_custom_protocol(
    builder: WebViewBuilder,
    local_root: WryLocalRoot,
    csp: Option<Csp>,
    request_sender: WryRequestSender,
) -> WebViewBuilder {
    let local_root = local_root.0;
    builder.with_asynchronous_custom_protocol("flurx".to_string(), move |_, request, responder| {
        let path = request.uri().path();
        let path = if path == "/" {
            "index.html"
        } else {
            &path[1..]
        };
        if let Err(e) = request_sender.0.send(WryRequest {
            responder,
            path: local_root.join(path),
            csp: csp.clone(),
        }) {
            error!("{e}");
        }
    })
}
