use crate::error::ApiResult;
use crate::fs::{AllowPaths, BaseDirectory, error_if_not_accessible, join_path_if_need};
use crate::macros::api_plugin;
use bevy::prelude::{In, Res};
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

api_plugin!(
    /// You'll be able to copy file from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const source = "./hello.txt"
    /// const destination = "./destination/hello.txt"
    /// await window.__FLURX__.fs.copyFile(source, destination)
    /// ```
    FsCopyFilePlugin,
    command: copy_file
);

#[derive(Deserialize, Default)]
struct Args {
    from: PathBuf,
    to: PathBuf,
    #[serde(rename = "fromBaseDir")]
    from_base_dir: Option<BaseDirectory>,
    #[serde(rename = "toBaseDir")]
    to_base_dir: Option<BaseDirectory>,
}

#[command(id = "FLURX|fs::copy_file")]
fn copy_file(In(args): In<Args>) -> Action<Args, ApiResult> {
    once::run(copy_file_system).with(args)
}

fn copy_file_system(In(args): In<Args>, scope: Option<Res<AllowPaths>>) -> ApiResult {
    let from = join_path_if_need(&args.from_base_dir, args.from);
    let to = join_path_if_need(&args.to_base_dir, args.to);
    error_if_not_accessible(&from, &scope)?;
    error_if_not_accessible(&to, &scope)?;
    std::fs::copy(from, to)?;
    Ok(())
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::AllowPaths;
    use crate::fs::copy_file::{Args, copy_file_system};
    use crate::tests::test_app;
    use bevy::prelude::*;
    use bevy::utils::default;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::Reactor;

    #[test]
    fn test_copy_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let from = tmp_dir.join("source0.txt");
                let to = tmp_dir.join("dest0.txt");
                std::fs::write(&from, "hello").unwrap();
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(copy_file_system).with(Args {
                            from,
                            to,
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap();
            }));
        });
        app.update();
    }

    #[test]
    fn copy_file_not_permitted_access() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                // Access to any files is not permitted.
                task.will(Update, once::res::insert().with(AllowPaths::default()))
                    .await;
                let tmp_dir = std::env::temp_dir();
                let from = tmp_dir.join("source1.txt");
                let to = tmp_dir.join("dest1.txt");
                std::fs::write(&from, "hello").unwrap();
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(copy_file_system).with(Args {
                            from,
                            to,
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap_err();
            }));
        });
        app.update();
        app.update();
    }

    #[test]
    fn copy_file_permitted_access() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                // Access to any files is not permitted.
                task.will(
                    Update,
                    once::res::insert().with(AllowPaths::new([tmp_dir.clone()])),
                )
                .await;
                let from = tmp_dir.join("source2.txt");
                let to = tmp_dir.join("dest2.txt");
                std::fs::write(&from, "hello").unwrap();
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(copy_file_system).with(Args {
                            from,
                            to,
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap();
            }));
        });
        app.update();
        app.update();
    }
}
