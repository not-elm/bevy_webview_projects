use crate::fs::{error_if_not_accessible, FsScope};
use crate::macros::define_api_plugin;
use bevy_ecs::system::{In, Res};
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;
use serde::Deserialize;


define_api_plugin!(
    /// You'll be able to remove a dir from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.fs.removeDir("./dir", {
    ///     recursive: true
    /// })
    /// ```
    FsRemoveDirPlugin,
    command: remove_dir
);

#[derive(Deserialize)]
struct Args {
    path: String,
    recursive: Option<bool>,
}

#[command(id = "FLURX|fs::remove_dir", internal)]
fn remove_dir(In(args): In<Args>) -> Action<Args, Result<(), String>> {
    once::run(remove_dir_system).with(args)
}

fn remove_dir_system(
    In(args): In<Args>,
    scope: Option<Res<FsScope>>,
) -> Result<(), String> {
    error_if_not_accessible(&args.path, &scope)?;
    if args.recursive.is_some_and(|recursive| recursive) {
        std::fs::remove_dir_all(args.path).map_err(|e| e.to_string())?;
    } else {
        std::fs::remove_dir(args.path).map_err(|e| e.to_string())?;
    }
    Ok(())
}


#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::remove_dir::{remove_dir_system, Args};
    use crate::tests::test_app;
    use bevy_app::{Startup, Update};
    use bevy_ecs::prelude::Commands;
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
                let result: Result<_, _> = task.will(Update, once::run(remove_dir_system).with(Args {
                    recursive: None,
                    path: empty_dir.to_str().unwrap().to_string(),
                })).await;
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
                let result: Result<_, _> = task.will(Update, once::run(remove_dir_system).with(Args {
                    recursive: None,
                    path: empty_dir.to_str().unwrap().to_string(),
                })).await;
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
                let result: Result<_, _> = task.will(Update, once::run(remove_dir_system).with(Args {
                    recursive: Some(true),
                    path: dir.to_str().unwrap().to_string(),
                })).await;
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