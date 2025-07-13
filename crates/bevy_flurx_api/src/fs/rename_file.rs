use crate::error::ApiResult;
use crate::fs::{AllowPaths, BaseDirectory, error_if_not_accessible, join_path_if_need};
use crate::macros::api_plugin;
use bevy::prelude::{In, Res};
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

api_plugin!(
    /// You'll be able to rename a file from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.fs.renameFile("./old.txt", "./new.txt", {
    ///     oldDir: "Download",
    ///     newDir: "Download",
    /// });
    /// ```
    FsRenameFilePlugin,
    command: rename_file
);

#[derive(Deserialize, Default)]
struct Args {
    #[serde(rename = "oldPath")]
    old_path: PathBuf,
    #[serde(rename = "newPath")]
    new_path: PathBuf,
    #[serde(rename = "oldDir")]
    old_dir: Option<BaseDirectory>,
    #[serde(rename = "newDir")]
    new_dir: Option<BaseDirectory>,
}

#[command(id = "FLURX|fs::rename_file")]
fn rename_file(In(args): In<Args>) -> Action<Args, ApiResult> {
    once::run(rename_file_system).with(args)
}

fn rename_file_system(In(args): In<Args>, scope: Option<Res<AllowPaths>>) -> ApiResult {
    let old_path = join_path_if_need(&args.old_dir, args.old_path);
    let new_path = join_path_if_need(&args.new_dir, args.new_path);
    error_if_not_accessible(&old_path, &scope)?;
    error_if_not_accessible(&new_path, &scope)?;
    std::fs::rename(old_path, new_path)?;
    Ok(())
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::AllowPaths;
    use crate::fs::rename_file::{Args, rename_file_system};
    use crate::tests::test_app;
    use bevy::prelude::*;
    use bevy::utils::default;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::{Reactor, Then};

    #[test]
    fn test_remove_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("rename_file_old1.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let new_path = tmp_dir.join("rename_file_new1.txt");
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(rename_file_system).with(Args {
                            old_path: hoge_path.clone(),
                            new_path: new_path.clone(),
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap();
                assert!(!std::fs::exists(hoge_path).unwrap());
                assert!(std::fs::exists(new_path).unwrap());
            }));
        });
        app.update();
    }

    #[test]
    fn error_out_of_scope() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("rename_file_old2.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let new_path = tmp_dir.join("rename_file_new2.txt");
                let result: Result<_, _> = task
                    .will(Update, {
                        once::res::insert().with(AllowPaths::default()).then(
                            once::run(rename_file_system).with(Args {
                                old_path: hoge_path.clone(),
                                new_path: new_path.clone(),
                                ..default()
                            }),
                        )
                    })
                    .await;
                result.unwrap_err();
                assert!(std::fs::exists(hoge_path).unwrap());
                assert!(!std::fs::exists(new_path).unwrap());
            }));
        });
        app.update();
    }
}
