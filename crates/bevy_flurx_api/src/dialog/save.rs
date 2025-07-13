use crate::dialog::DialogFilter;
use crate::fs::AllowPaths;
use crate::macros::api_plugin;
use bevy::prelude::{In, ResMut, Update};
use bevy_flurx::action::once;
use bevy_flurx::task::ReactorTask;
use bevy_flurx_ipc::prelude::*;
use rfd::FileDialog;
use serde::Deserialize;
use std::path::PathBuf;

api_plugin!(
    /// You'll be able to open a file save dialog.
    ///
    ///  The selected path will be registered to [AllowPaths] until the application closed.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.dialog.save("question");
    /// ```
    DialogSavePlugin,
    command: save
);

#[derive(Default, Deserialize)]
struct Args {
    title: Option<String>,
    #[serde(rename = "defaultPath")]
    default_path: Option<String>,
    filters: Option<Vec<DialogFilter>>,
}

#[command(id = "FLURX|dialog::save")]
async fn save(In(args): In<Args>, task: ReactorTask) -> Option<PathBuf> {
    let path = select_save_path(args);
    task.will(Update, once::run(save_system).with(path.clone()))
        .await;
    path
}

fn save_system(In(path): In<Option<PathBuf>>, allows: Option<ResMut<AllowPaths>>) {
    if let Some(mut allows) = allows {
        if let Some(path) = path {
            allows.add(path);
        }
    }
}

fn select_save_path(args: Args) -> Option<PathBuf> {
    let mut dialog = FileDialog::new();
    dialog = dialog.set_can_create_directories(true);
    if let Some(title) = args.title {
        dialog = dialog.set_title(title);
    }
    if let Some(default_path) = args.default_path {
        dialog = dialog.set_directory(default_path);
    }
    if let Some(filters) = args.filters {
        for filter in filters {
            dialog = dialog.add_filter(filter.name, &filter.extensions);
        }
    }
    dialog.save_file()
}
