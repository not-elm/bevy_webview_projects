use crate::dialog::DialogFilter;
use crate::fs::AllowPaths;
use crate::macros::api_plugin;
use bevy::prelude::{In, ResMut};
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

api_plugin!(
    /// You'll be able to open a file/directory selection dialog.
    ///
    ///  The selected path will be registered to [AllowPaths] until the application closed.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.dialog.open();
    /// ```
    DialogOpenPlugin,
    command: open
);

#[derive(Default, Deserialize)]
struct Args {
    title: Option<String>,
    #[serde(rename = "defaultPath")]
    default_path: Option<String>,
    directory: Option<bool>,
    multiple: Option<bool>,
    filters: Option<Vec<DialogFilter>>,
}

#[derive(Serialize, Debug)]
enum SelectedPaths {
    Single(Option<PathBuf>),
    Multiple(Option<Vec<PathBuf>>),
}

#[command(id = "FLURX|dialog::open")]
fn open(In(args): In<Args>) -> Action<Args, SelectedPaths> {
    once::run(open_system).with(args)
}

fn open_system(In(args): In<Args>, allows: Option<ResMut<AllowPaths>>) -> SelectedPaths {
    let paths = select_paths(args);
    if let Some(mut allows) = allows {
        match &paths {
            SelectedPaths::Single(Some(path)) => {
                allows.add(path.clone());
            }
            SelectedPaths::Multiple(Some(paths)) => {
                allows.add_all(paths.clone());
            }
            _ => {}
        }
    }
    paths
}

fn select_paths(args: Args) -> SelectedPaths {
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
    match (
        args.directory.unwrap_or(false),
        args.multiple.unwrap_or(false),
    ) {
        (true, true) => SelectedPaths::Multiple(dialog.pick_folders()),
        (true, false) => SelectedPaths::Single(dialog.pick_folder()),
        (false, true) => SelectedPaths::Multiple(dialog.pick_files()),
        (false, false) => SelectedPaths::Single(dialog.pick_file()),
    }
}
