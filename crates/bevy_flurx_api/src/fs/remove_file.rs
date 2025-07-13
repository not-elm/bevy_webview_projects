use crate::error::ApiResult;
use crate::fs::{AllowPaths, BaseDirectory, error_if_not_accessible, join_path_if_need};
use crate::macros::api_plugin;
use bevy::prelude::{In, Res};
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

api_plugin!(
    /// You'll be able to remove file from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.fs.removeFile("./hoge.txt", {
    ///     dir: "Download"
    /// });
    /// ```
    FsRemoveFilePlugin,
    command: remove_file
);

#[derive(Deserialize, Default)]
struct Args {
    path: PathBuf,
    dir: Option<BaseDirectory>,
}

#[command(id = "FLURX|fs::remove_file")]
fn remove_file(In(args): In<Args>) -> Action<Args, ApiResult> {
    once::run(remove_file_system).with(args)
}

fn remove_file_system(In(args): In<Args>, scope: Option<Res<AllowPaths>>) -> ApiResult {
    let path = join_path_if_need(&args.dir, args.path);
    error_if_not_accessible(&path, &scope)?;
    std::fs::remove_file(path)?;
    Ok(())
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::AllowPaths;
    use crate::fs::remove_file::{Args, remove_file_system};
    use crate::tests::test_app;
    use bevy::prelude::*;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::{Reactor, Then};

    #[test]
    fn test_remove_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("remove_file_hoge1.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(remove_file_system).with(Args {
                            path: hoge_path.clone(),
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap();
                assert!(!std::fs::exists(hoge_path).unwrap());
            }));
        });
        app.update();
    }

    #[test]
    fn err_if_out_of_scope() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("remove_file_hoge2.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let result: Result<_, _> = task
                    .will(Update, {
                        once::res::insert().with(AllowPaths::default()).then(
                            once::run(remove_file_system).with(Args {
                                path: hoge_path.clone(),
                                ..default()
                            }),
                        )
                    })
                    .await;
                result.unwrap_err();
                assert!(std::fs::exists(hoge_path).unwrap());
            }));
        });
        app.update();
    }
}
