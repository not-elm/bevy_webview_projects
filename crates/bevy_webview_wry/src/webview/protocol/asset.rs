use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, Handle, LoadContext};
use bevy::prelude::*;
use bevy_webview_core::prelude::Csp;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use wry::http::header::{CONTENT_SECURITY_POLICY, CONTENT_TYPE};
use wry::http::Response;

#[derive(Debug, Component, Clone)]
pub struct WryResponseHandle(pub Handle<WryResponseBody>);

#[derive(Debug, Asset, TypePath)]
pub struct WryResponseBody(pub Vec<u8>);

#[derive(
    Serialize, Deserialize, Default, Eq, PartialEq, Hash, Component, Reflect, Clone, Debug,
)]
#[reflect(Component, Serialize, Deserialize)]
pub struct WryRequestArgs {
    pub csp: Option<Csp>,
    pub path: PathBuf,
}

#[derive(Default)]
pub struct WryResponseLoader;

impl AssetLoader for WryResponseLoader {
    type Asset = WryResponseBody;
    type Settings = ();
    type Error = std::io::Error;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        _: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        Ok(WryResponseBody(buf))
    }

    fn extensions(&self) -> &[&str] {
        &EXTENSIONS
    }
}

pub(crate) fn convert_to_response(content: Vec<u8>, args: &WryRequestArgs) -> Response<Vec<u8>> {
    try_convert_to_response(content, args).unwrap_or_else(|e| {
        Response::builder()
            .header(CONTENT_TYPE, "text/plain")
            .status(500)
            .body(e.to_string().as_bytes().to_vec())
            .unwrap()
    })
}

fn try_convert_to_response(
    content: Vec<u8>,
    args: &WryRequestArgs,
) -> Result<Response<Vec<u8>>, std::io::Error> {
    let Some(mimetype) = get_mime_type(&args.path) else {
        return Err(std::io::Error::other(Box::new(NotImplError(
            args.path.clone(),
        ))));
    };
    let mut response_builder = Response::builder();
    if let Some(csp) = args.csp.as_ref() {
        response_builder = response_builder.header(CONTENT_SECURITY_POLICY, csp.0.as_str());
    }
    response_builder
        .header(CONTENT_TYPE, mimetype)
        .body(content)
        .map_err(std::io::Error::other)
}

struct NotImplError(PathBuf);
impl Display for NotImplError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("not implemented content type {:?}", self.0))
    }
}

impl Debug for NotImplError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self}"))
    }
}

impl std::error::Error for NotImplError {}

const EXTENSION_MAP: &[(&[&str], &str)] = &[
    (&["htm", "html"], "text/html"),
    (&["txt"], "text/plain"),
    (&["css"], "text/css"),
    (&["csv"], "text/csv"),
    (&["js"], "text/javascript"),
    (&["jpeg", "jpg"], "image/jpeg"),
    (&["png"], "image/png"),
    (&["gif"], "image/gif"),
    (&["bmp"], "image/bmp"),
    (&["svg"], "image/svg+xml"),
    (&["json"], "application/json"),
    (&["pdf"], "application/pdf"),
    (&["zip"], "application/zip"),
    (&["lzh"], "application/x-lzh"),
    (&["tar"], "application/x-tar"),
    (&["wasm"], "application/wasm"),
    (&["mp3"], "audio/mp3"),
    (&["mp4"], "video/mp4"),
    (&["mpeg"], "video/mpeg"),
    (&["aac"], "audio/aac"),
    (&["abw"], "application/x-abiword"),
    (&["arc"], "application/x-freearc"),
    (&["avi"], "video/m-msvideo"),
    (&["azw"], "application/vnd.amazon.ebook"),
    (&["bin"], "application/octet-stream"),
    (&["bz"], "application/x-bzip"),
    (&["bz2"], "application/x-bzip2"),
    (&["csh"], "application/x-csh"),
    (&["doc"], "application/msword"),
    (
        &["docx"],
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    ),
    (&["eot"], "application/vnd.ms-fontobject"),
    (&["epub"], "application/epub+zip"),
    (&["gz"], "application/gzip"),
    (&["ico"], "image/vnd.microsoft.icon"),
    (&["ics"], "text/calendar"),
    (&["jar"], "application/java-archive"),
    (&["jpeg", "jpg"], "image/jpeg"),
    (&["mid", "midi"], "audio/midi"),
    (&["mpkg"], "application/vnd.apple.installer+xml"),
    (&["odp"], "application/vnd.oasis.opendocument.presentation"),
    (&["ods"], "application/vnd.oasis.opendocument.spreadsheet"),
    (&["odt"], "application/vnd.oasis.opendocument.text"),
    (&["oga"], "audio/ogg"),
    (&["ogv"], "video/ogg"),
    (&["ogx"], "application/ogg"),
    (&["otf"], "font/otf"),
    (&["ppt"], "application/vnd.ms-powerpoint"),
    (
        &["pptx"],
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    ),
    (&["rar"], "application/vnd.rar"),
    (&["rtf"], "application/rtf"),
    (&["sh"], "application/x-sh"),
    (&["swf"], "application/x-shockwave-flash"),
    (&["tif", "tiff"], "image/tiff"),
    (&["ttf"], "font/ttf"),
    (&["vsd"], "application/vnd.visio"),
    (&["wav"], "audio/wav"),
    (&["weba"], "audio/webm"),
    (&["webm"], "video/web"),
    (&["woff"], "font/woff"),
    (&["woff2"], "font/woff2"),
    (&["xhtml"], "application/xhtml+xml"),
    (&["xls"], "application/vnd.ms-excel"),
    (
        &["xlsx"],
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    ),
    (&["xml"], "application/xml"),
    (&["xul"], "application/vnd.mozilla.xul+xml"),
    (&["7z"], "application/x-7z-compressed"),
];

static EXTENSIONS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    EXTENSION_MAP
        .iter()
        .flat_map(|(extensions, _)| *extensions)
        .copied()
        .collect::<Vec<&str>>()
});

fn get_mime_type(path: &Path) -> Option<&str> {
    let ext = path.extension()?.to_str()?;
    EXTENSION_MAP
        .iter()
        .find(|(extensions, _)| extensions.iter().any(|e| e == &ext))
        .map(|(_, mime)| *mime)
}
