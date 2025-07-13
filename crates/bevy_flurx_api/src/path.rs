//! Provides apis to obtain special system paths.

use crate::fs::{AllowPaths, error_if_not_accessible};
use crate::macros::api_plugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::{PluginGroup, Res};
use bevy_flurx::action::once;
use bevy_flurx::prelude::ActionSeed;
use bevy_flurx_ipc::prelude::*;
use std::path::PathBuf;

/// Allows you to use all path bevy_flurx_api plugins.
///
/// ## Plugins
///
/// - [PathConfigPlugin]
/// - [PathConfigLocalPlugin]
/// - [PathDataPlugin]
/// - [PathDataLocalPlugin]
/// - [PathAudioPlugin]
/// - [PathCachePlugin]
/// - [PathDesktopPlugin]
/// - [PathDocumentPlugin]
/// - [PathDownloadPlugin]
/// - [PathExecutablePlugin]
/// - [PathPublicPlugin]
/// - [PathRuntimePlugin]
/// - [PathTempPlugin]
/// - [PathTemplatePlugin]
/// - [PathVideoPlugin]
/// - [PathHomePlugin]
/// - [PathPicturePlugin]
/// - [PathFontPlugin]
pub struct AllPathPlugins;
impl PluginGroup for AllPathPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PathConfigPlugin)
            .add(PathConfigLocalPlugin)
            .add(PathDataPlugin)
            .add(PathDataLocalPlugin)
            .add(PathAudioPlugin)
            .add(PathCachePlugin)
            .add(PathDesktopPlugin)
            .add(PathDocumentPlugin)
            .add(PathDownloadPlugin)
            .add(PathExecutablePlugin)
            .add(PathPublicPlugin)
            .add(PathRuntimePlugin)
            .add(PathTempPlugin)
            .add(PathTemplatePlugin)
            .add(PathVideoPlugin)
            .add(PathHomePlugin)
            .add(PathPicturePlugin)
            .add(PathFontPlugin)
    }
}

api_plugin!(
    /// You'll be able to obtain user's config path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.cache();
    /// ```
    PathConfigPlugin,
    command: config
);

api_plugin!(
    /// You'll be able to obtain user's config local path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.configLocal();
    /// ```
    PathConfigLocalPlugin,
    command: config_local
);

api_plugin!(
    /// You'll be able to obtain user's data path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.data_local();
    /// ```
    PathDataLocalPlugin,
    command: data_local
);

api_plugin!(
    /// You'll be able to obtain user's data path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.data();
    /// ```
    PathDataPlugin,
    command: data
);

api_plugin!(
    /// You'll be able to obtain user's audio path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.audio();
    /// ```
    PathAudioPlugin,
    command: audio
);

api_plugin!(
    /// You'll be able to obtain user's cache path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.cache();
    /// ```
    PathCachePlugin,
    command: cache
);

api_plugin!(
    /// You'll be able to obtain user's desktop path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.desktop();
    /// ```
    PathDesktopPlugin,
    command: desktop
);

api_plugin!(
    /// You'll be able to obtain user's document path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.document();
    /// ```
    PathDocumentPlugin,
    command: document
);

api_plugin!(
    /// You'll be able to obtain user's download path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.download();
    /// ```
    PathDownloadPlugin,
    command: download
);

api_plugin!(
    /// You'll be able to obtain user's executable path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.executable();
    /// ```
    PathExecutablePlugin,
    command: executable
);

api_plugin!(
    /// You'll be able to obtain user's public path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.public();
    /// ```
    PathPublicPlugin,
    command: public
);

api_plugin!(
    /// You'll be able to obtain user's runtime path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.runtime();
    /// ```
    PathRuntimePlugin,
    command: runtime
);

api_plugin!(
    /// You'll be able to obtain user's temp path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.temp();
    /// ```
    PathTempPlugin,
    command: temp
);

api_plugin!(
    /// You'll be able to obtain user's template path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.template();
    /// ```
    PathTemplatePlugin,
    command: template
);

api_plugin!(
    /// You'll be able to obtain user's video path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.video();
    /// ```
    PathVideoPlugin,
    command: video
);

api_plugin!(
    /// You'll be able to obtain user's home path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.home();
    /// ```
    PathHomePlugin,
    command: home
);

api_plugin!(
    /// You'll be able to obtain user's picture path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.picture();
    /// ```
    PathPicturePlugin,
    command: picture
);

api_plugin!(
    /// You'll be able to obtain user's font path from typescript(or js).
    ///
    /// If the path doesn't exist or is not permitted by [AllowPaths], will be null.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.font();
    /// ```
    PathFontPlugin,
    command: font
);

#[command(id = "FLURX|path::config")]
fn config() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::config_dir))
}

#[command(id = "FLURX|path::config_local")]
fn config_local() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::config_local_dir))
}

#[command(id = "FLURX|path::data")]
fn data() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::data_dir))
}

#[command(id = "FLURX|path::data_local")]
fn data_local() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::data_local_dir))
}

#[command(id = "FLURX|path::audio")]
fn audio() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::audio_dir))
}

#[command(id = "FLURX|path::cache")]
fn cache() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::cache_dir))
}

#[command(id = "FLURX|path::desktop")]
fn desktop() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::desktop_dir))
}

#[command(id = "FLURX|path::document")]
fn document() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::document_dir))
}

#[command(id = "FLURX|path::download")]
fn download() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::download_dir))
}

#[command(id = "FLURX|path::executable")]
fn executable() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::executable_dir))
}

#[command(id = "FLURX|path::public")]
fn public() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::public_dir))
}

#[command(id = "FLURX|path::runtime")]
fn runtime() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::runtime_dir))
}

#[command(id = "FLURX|path::temp")]
fn temp() -> ActionSeed<(), Option<PathBuf>> {
    fn path() -> Option<PathBuf> {
        Some(std::env::temp_dir())
    }
    once::run(obtain_path(path))
}

#[command(id = "FLURX|path::template")]
fn template() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::template_dir))
}

#[command(id = "FLURX|path::video")]
fn video() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::video_dir))
}

#[command(id = "FLURX|path::home")]
fn home() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::home_dir))
}

#[command(id = "FLURX|path::picture")]
fn picture() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::picture_dir))
}

#[command(id = "FLURX|path::font")]
fn font() -> ActionSeed<(), Option<PathBuf>> {
    once::run(obtain_path(dirs::font_dir))
}

fn obtain_path(f: fn() -> Option<PathBuf>) -> impl Fn(Option<Res<AllowPaths>>) -> Option<PathBuf> {
    move |scope: Option<Res<AllowPaths>>| {
        let path = f()?;
        if error_if_not_accessible(&path, &scope).is_ok() {
            Some(path)
        } else {
            None
        }
    }
}
