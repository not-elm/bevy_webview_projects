use crate::error::ApiResult;
use crate::fs::{AllowPaths, BaseDirectory, error_if_not_accessible, join_path_if_need};
use crate::macros::api_plugin;
use bevy::prelude::{In, Res};
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::PathBuf;

api_plugin!(
    /// You'll be able to check if the path exists from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const entries: window.__FLURX__.fs.FileEntry[]= await window.__FLURX__.fs.readDir("./dir", {
    ///     dir: "Download"
    /// });
    /// ```
    FsReadDirPlugin,
    command: read_dir
);

#[derive(Deserialize, Default)]
struct Args {
    path: PathBuf,
    dir: Option<BaseDirectory>,
}

#[derive(Serialize, PartialEq, Debug)]
struct FileEntry {
    name: OsString,
    path: PathBuf,
    // none if not dir.
    children: Option<Vec<FileEntry>>,
}

#[command(id = "FLURX|fs::read_dir")]
fn read_dir(In(args): In<Args>) -> Action<Args, ApiResult<Vec<FileEntry>>> {
    once::run(read_dir_system).with(args)
}

fn read_dir_system(
    In(args): In<Args>,
    scope: Option<Res<AllowPaths>>,
) -> ApiResult<Vec<FileEntry>> {
    let path = join_path_if_need(&args.dir, args.path);
    error_if_not_accessible(&path, &scope)?;
    Ok(read_dirs(&path)?)
}

fn read_dirs(path: &PathBuf) -> std::io::Result<Vec<FileEntry>> {
    let mut file_entries = Vec::new();
    for entry in std::fs::read_dir(path)?.filter_map(|entry| entry.ok()) {
        let children = if entry.metadata()?.is_dir() {
            Some(read_dirs(&entry.path())?)
        } else {
            None
        };
        file_entries.push(FileEntry {
            name: entry.file_name(),
            path: entry.path(),
            children,
        });
    }
    Ok(file_entries)
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::read_dir::{Args, FileEntry, read_dir_system};
    use crate::tests::test_app;
    use bevy::prelude::*;
    use bevy::utils::default;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::Reactor;
    use std::ffi::OsString;
    use std::fs::{create_dir, create_dir_all};

    #[test]
    fn empty_dir() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir().join("read_dir_empty_dir");
                let _ = create_dir(&tmp_dir);
                let entries: Vec<FileEntry> = task
                    .will(
                        Update,
                        once::run(read_dir_system).with(Args {
                            path: tmp_dir,
                            ..default()
                        }),
                    )
                    .await
                    .unwrap();
                assert!(entries.is_empty());
            }));
        });
        app.update();
    }

    #[test]
    fn read_text_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir().join("read_dir_text_dir");
                let _ = create_dir(&tmp_dir);
                let _ = std::fs::write(tmp_dir.join("hello.txt"), "hello");
                let entries: Vec<FileEntry> = task
                    .will(
                        Update,
                        once::run(read_dir_system).with(Args {
                            path: tmp_dir.clone(),
                            ..default()
                        }),
                    )
                    .await
                    .unwrap();
                assert_eq!(
                    entries,
                    vec![FileEntry {
                        path: tmp_dir.join("hello.txt"),
                        name: OsString::from("hello.txt"),
                        children: None,
                    }]
                );
            }));
        });
        app.update();
    }

    #[test]
    fn contains_dir() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir().join("read_dir_contains_dir");
                let _ = create_dir_all(tmp_dir.join("child"));
                let entries: Vec<FileEntry> = task
                    .will(
                        Update,
                        once::run(read_dir_system).with(Args {
                            path: tmp_dir.clone(),
                            ..default()
                        }),
                    )
                    .await
                    .unwrap();
                assert_eq!(
                    entries,
                    vec![FileEntry {
                        path: tmp_dir.join("child"),
                        name: OsString::from("child"),
                        children: Some(vec![]),
                    }]
                );
            }));
        });
        app.update();
    }
}
