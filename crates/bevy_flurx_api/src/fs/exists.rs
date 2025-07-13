use crate::error::ApiResult;
use crate::fs::{AllowPaths, BaseDirectory, error_if_not_accessible, join_path_if_need};
use crate::macros::api_plugin;
use bevy::prelude::{In, Res};
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

api_plugin!(
    /// You'll be able to check if the path exists from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const existsPath: boolean = await window.__FLURX__.fs.exists("./dir");
    /// ```
    FsExistsPlugin,
    command: exists
);

#[derive(Deserialize, Default)]
struct Args {
    path: PathBuf,
    dir: Option<BaseDirectory>,
}

#[command(id = "FLURX|fs::exists")]
fn exists(In(args): In<Args>) -> Action<Args, ApiResult<bool>> {
    once::run(exists_system).with(args)
}

fn exists_system(In(args): In<Args>, scope: Option<Res<AllowPaths>>) -> ApiResult<bool> {
    let path = join_path_if_need(&args.dir, args.path);
    error_if_not_accessible(&path, &scope)?;
    Ok(std::fs::exists(path)?)
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::AllowPaths;
    use crate::fs::exists::{Args, exists_system};
    use crate::tests::test_app;
    use bevy::prelude::*;
    use bevy::utils::default;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::{Reactor, Then};

    #[test]
    fn test_exists() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(exists_system).with(Args {
                            path: tmp_dir,
                            ..default()
                        }),
                    )
                    .await;
                assert!(result.unwrap());
            }));
        });
        app.update();
    }

    #[test]
    fn test_not_exists() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let not_exists_dir = tmp_dir.join("not_exists");
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(exists_system).with(Args {
                            path: not_exists_dir,
                            ..default()
                        }),
                    )
                    .await;
                assert!(!result.unwrap());
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
                let result: Result<_, _> = task
                    .will(Update, {
                        once::res::insert().with(AllowPaths::default()).then(
                            once::run(exists_system).with(Args {
                                path: tmp_dir,
                                ..default()
                            }),
                        )
                    })
                    .await;
                result.unwrap_err();
            }));
        });
        app.update();
    }
}
