use crate::error::ApiResult;
use crate::fs::{AllowPaths, BaseDirectory, error_if_not_accessible, join_path_if_need};
use crate::macros::api_plugin;
use bevy::prelude::{In, Res};
use bevy_flurx::action::{Action, once};
use bevy_flurx_ipc::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

api_plugin!(
    /// You'll be able to read a file as a UTF-8 encoded string from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const contents: string = await window.__FLURX__.fs.readTextFile("./dir",{
    ///     dir: "Download"
    /// });
    /// ```
    FsReadTextFilePlugin,
    command: read_text_file
);

api_plugin!(
    /// You'll be able to read a file from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const contents: string = await window.__FLURX__.fs.readBinaryFile("./dir",{
    ///     dir: "Download"
    /// });
    /// ```
    FsReadBinaryFilePlugin,
    command: read_binary_file
);

#[derive(Deserialize, Default)]
struct Args {
    path: PathBuf,
    dir: Option<BaseDirectory>,
}

#[command(id = "FLURX|fs::read_text_file")]
fn read_text_file(In(args): In<Args>) -> Action<Args, ApiResult<String>> {
    once::run(read_text_file_system).with(args)
}

#[command(id = "FLURX|fs::read_binary_file")]
fn read_binary_file(In(args): In<Args>) -> Action<Args, ApiResult<Vec<u8>>> {
    once::run(read_binary_file_system).with(args)
}

fn read_binary_file_system(
    In(args): In<Args>,
    scope: Option<Res<AllowPaths>>,
) -> ApiResult<Vec<u8>> {
    let path = join_path_if_need(&args.dir, args.path);
    error_if_not_accessible(&path, &scope)?;
    Ok(std::fs::read(path)?)
}

fn read_text_file_system(In(args): In<Args>, scope: Option<Res<AllowPaths>>) -> ApiResult<String> {
    let path = join_path_if_need(&args.dir, args.path);
    error_if_not_accessible(&path, &scope)?;
    Ok(std::fs::read_to_string(path)?)
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::AllowPaths;
    use crate::fs::read_file::{Args, read_text_file_system};
    use crate::tests::test_app;
    use bevy::prelude::*;
    use bevy::utils::default;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::{Reactor, Then};

    #[test]
    fn test_read_text_from_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("read_text_file_read_text_file.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let result: Result<String, _> = task
                    .will(
                        Update,
                        once::run(read_text_file_system).with(Args {
                            path: hoge_path,
                            ..default()
                        }),
                    )
                    .await;
                assert_eq!(result.unwrap(), "hoge");
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
                let hoge_path = tmp_dir.join("read_text_file_read_text_file2.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let result: Result<String, _> = task
                    .will(Update, {
                        once::res::insert().with(AllowPaths::default()).then(
                            once::run(read_text_file_system).with(Args {
                                path: hoge_path,
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
