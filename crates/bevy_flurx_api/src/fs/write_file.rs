use crate::error::ApiResult;
use crate::fs::{AllowPaths, BaseDirectory, error_if_not_accessible, join_path_if_need};
use crate::macros::api_plugin;
use bevy::prelude::{In, Res};
use bevy_flurx::action::{Action, once};
use bevy_flurx::prelude::Pipe;
use bevy_flurx_ipc::prelude::*;
use serde::Deserialize;
use std::io::Write;
use std::path::PathBuf;

api_plugin!(
    /// You'll be able to write a file from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.fs.writeBinaryFile("./hoge.txt", new Uint8Array([0, 1, 2]), {
    ///     dir: "Download",
    ///     append: true,
    ///     recursive: true
    /// })
    /// ```
    FsWriteBinaryFilePlugin,
    command: write_binary_file
);

api_plugin!(
    /// You'll be able to write a file from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.fs.writeTextFile("./hoge.txt", "file contents", {
    ///     dir: "Download",
    ///     append: true,
    ///     recursive: true
    /// })
    /// ```
    FsWriteTextFilePlugin,
    command: write_text_file
);

#[derive(Deserialize, Default)]
struct BinaryFileArgs {
    path: PathBuf,
    contents: Vec<u8>,
    dir: Option<BaseDirectory>,
    append: Option<bool>,
    recursive: Option<bool>,
}

#[derive(Deserialize, Default)]
struct TextFileArgs {
    path: PathBuf,
    contents: String,
    dir: Option<BaseDirectory>,
    append: Option<bool>,
    recursive: Option<bool>,
}

#[command(id = "FLURX|fs::write_binary_file")]
fn write_binary_file(In(args): In<BinaryFileArgs>) -> Action<BinaryFileArgs, ApiResult> {
    once::run(write_file_system).with(args)
}

#[command(id = "FLURX|fs::write_text_file")]
fn write_text_file(In(args): In<TextFileArgs>) -> Action<TextFileArgs, ApiResult> {
    once::run(|In(args): In<TextFileArgs>| BinaryFileArgs {
        path: args.path,
        contents: args.contents.into_bytes(),
        dir: args.dir,
        append: args.append,
        recursive: args.recursive,
    })
    .pipe(once::run(write_file_system))
    .with(args)
}

fn write_file_system(In(args): In<BinaryFileArgs>, scope: Option<Res<AllowPaths>>) -> ApiResult {
    let path = join_path_if_need(&args.dir, args.path);
    error_if_not_accessible(&path, &scope)?;
    let append = args.append.is_some_and(|append| append);
    if args.recursive.is_some_and(|recursive| recursive) {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
    }

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(path)?;
    file.write_all(&args.contents)?;
    Ok(())
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::write_file::{BinaryFileArgs, write_file_system};
    use crate::tests::test_app;
    use bevy::prelude::*;
    use bevy::utils::default;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::Reactor;

    #[test]
    fn overwrite_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("write_file_hoge1.txt");
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(write_file_system).with(BinaryFileArgs {
                            path: hoge_path.clone(),
                            contents: b"hoge".to_vec(),
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap();
                assert_eq!(std::fs::read_to_string(hoge_path).unwrap(), "hoge");
            }));
        });
        app.update();
    }

    #[test]
    fn append_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("write_file_hoge2.txt");
                let _ = std::fs::remove_file(&hoge_path);
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(write_file_system).with(BinaryFileArgs {
                            path: hoge_path.clone(),
                            contents: b"hoge".to_vec(),
                            append: Some(true),
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap();
                assert_eq!(std::fs::read_to_string(&hoge_path).unwrap(), "hoge");
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(write_file_system).with(BinaryFileArgs {
                            path: hoge_path.clone(),
                            contents: b"hoge".to_vec(),
                            append: Some(true),
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap();
                assert_eq!(std::fs::read_to_string(&hoge_path).unwrap(), "hogehoge");
            }));
        });
        app.update();
    }

    #[test]
    fn err_if_not_specified_recursive() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("not_exists_dir").join("hoge1.txt");
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(write_file_system).with(BinaryFileArgs {
                            path: hoge_path.clone(),
                            contents: b"hoge".to_vec(),
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap_err();
            }));
        });
        app.update();
    }

    #[test]
    fn recursive_create_dir() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("write_file").join("hoge2.txt");
                let result: Result<_, _> = task
                    .will(
                        Update,
                        once::run(write_file_system).with(BinaryFileArgs {
                            path: hoge_path.clone(),
                            contents: b"hoge".to_vec(),
                            recursive: Some(true),
                            ..default()
                        }),
                    )
                    .await;
                result.unwrap();
                assert_eq!(std::fs::read_to_string(hoge_path).unwrap(), "hoge");
            }));
        });
        app.update();
    }
}
