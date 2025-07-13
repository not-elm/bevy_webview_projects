use crate::error::ApiResult;
use crate::fs::{AllowPaths, BaseDirectory, error_if_not_accessible, join_path_if_need};
use crate::macros::api_plugin;
use bevy::prelude::{In, Res};
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

api_plugin!(
    /// You'll be able to remove a dir from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.fs.removeDir("./dir", {
    ///      dir: "Download",
    ///     recursive: true
    /// })
    /// ```
    FsRemoveDirPlugin,
    command: remove_dir
);

#[derive(Deserialize, Default)]
struct Args {
    path: PathBuf,
    dir: Option<BaseDirectory>,
    recursive: Option<bool>,
}

#[command(id = "FLURX|fs::remove_dir")]
fn remove_dir(In(args): In<Args>) -> Action<Args, ApiResult> {
    once::run(remove_dir_system).with(args)
}

fn remove_dir_system(In(args): In<Args>, scope: Option<Res<AllowPaths>>) -> ApiResult {
    let path = join_path_if_need(&args.dir, args.path);
    error_if_not_accessible(&path, &scope)?;
    if args.recursive.is_some_and(|recursive| recursive) {
        std::fs::remove_dir_all(path)?;
    } else {
        std::fs::remove_dir(path)?;
    }
    Ok(())
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::remove_dir::{Args, remove_dir_system};
    use crate::tests::test_app;
    use bevy::prelude::*;
    use bevy::utils::default;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::Reactor;
    use std::path::PathBuf;

    #[test]
    fn remove_empty_dir() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let empty_dir = std::env::temp_dir().join("empty_dir");
                create_dir_if_need(&empty_dir);
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(remove_dir_system).with(Args {
                            path: empty_dir.clone(),
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap();
                assert!(!std::fs::exists(empty_dir).unwrap());
            }));
        });
        app.update();
    }

    #[test]
    fn err_if_not_empty_dir_and_without_recursive() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let empty_dir = std::env::temp_dir().join("not_empty_dir");
                create_dir_if_need(&empty_dir);
                create_dir_if_need(&empty_dir.join("dir"));
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(remove_dir_system).with(Args {
                            path: empty_dir.clone(),
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap_err();
                assert!(std::fs::exists(empty_dir).unwrap());
            }));
        });
        app.update();
    }

    #[test]
    fn remove_with_recursive() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let dir = std::env::temp_dir().join("not_empty_dir2");
                create_dir_if_need(&dir);
                create_dir_if_need(&dir.join("dir"));
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(remove_dir_system).with(Args {
                            recursive: Some(true),
                            path: dir.clone(),
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap();
                assert!(!std::fs::exists(dir).unwrap());
            }));
        });
        app.update();
    }

    fn create_dir_if_need(path: &PathBuf) {
        if !path.exists() {
            std::fs::create_dir_all(path).unwrap();
        }
    }
}
