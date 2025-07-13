//!  Provides mechanism to control a dialog from a webview.

mod open;
mod save;

use crate::macros::api_plugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::{In, PluginGroup};
use bevy_flurx::action::once;
use bevy_flurx::prelude::Action;
use bevy_flurx_ipc::prelude::*;
use rfd::{MessageButtons, MessageDialogResult, MessageLevel};
use serde::Deserialize;

pub use open::DialogOpenPlugin;
pub use save::DialogSavePlugin;

/// Allows you to use all dialog plugins
///
/// ## Plugins
///
/// - [DialogAskPlugin]
/// - [DialogConfirmPlugin]
/// - [DialogMessagePlugin]
/// - [DialogOpenPlugin]
/// - [DialogSavePlugin]
pub struct AllDialogPlugins;
impl PluginGroup for AllDialogPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(DialogAskPlugin)
            .add(DialogConfirmPlugin)
            .add(DialogMessagePlugin)
            .add(DialogOpenPlugin)
            .add(DialogSavePlugin)
    }
}

api_plugin!(
    /// You'll be able to control a dialog to ask the user yes/no from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const yes: bool = await window.__FLURX__.dialog.ask("question");
    /// ```
    DialogAskPlugin,
    command: ask
);

api_plugin!(
    /// You'll be able to control a dialog to confirm ok/cancel with the user from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const isOk: bool = await window.__FLURX__.dialog.confirm("question");
    /// ```
    DialogConfirmPlugin,
    command: confirm
);

api_plugin!(
    /// You'll be able to control a dialog to show a message dialog from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.dialog.message("question");
    /// ```
    DialogMessagePlugin,
    command: message
);

#[derive(Deserialize)]
struct DialogFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

#[derive(Default, Deserialize)]
struct Args {
    #[serde(rename = "questionMessage")]
    question_message: String,
    title: Option<String>,
    level: Option<DialogLevel>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum DialogLevel {
    Info,
    Warn,
    Error,
}

impl DialogLevel {
    fn to_message_level(&self) -> MessageLevel {
        match self {
            Self::Info => MessageLevel::Info,
            Self::Warn => MessageLevel::Warning,
            Self::Error => MessageLevel::Error,
        }
    }
}

#[command(id = "FLURX|dialog::ask")]
fn ask(In(args): In<Args>) -> Action<Args, bool> {
    once::run(|In(args): In<Args>| ask_system(args, MessageButtons::YesNo)).with(args)
}

#[command(id = "FLURX|dialog::confirm")]
fn confirm(In(args): In<Args>) -> Action<Args, bool> {
    once::run(|In(args): In<Args>| ask_system(args, MessageButtons::OkCancel)).with(args)
}

#[command(id = "FLURX|dialog::message")]
fn message(In(args): In<Args>) -> Action<Args, bool> {
    once::run(|In(args): In<Args>| ask_system(args, MessageButtons::Ok)).with(args)
}

fn ask_system(args: Args, buttons: MessageButtons) -> bool {
    let mut dialog = rfd::MessageDialog::new();
    if let Some(title) = args.title {
        dialog = dialog.set_title(title);
    }
    if let Some(dialog_type) = args.level {
        dialog = dialog.set_level(dialog_type.to_message_level());
    }

    let dialog_result = dialog
        .set_description(args.question_message)
        .set_buttons(buttons)
        .show();
    matches!(
        dialog_result,
        MessageDialogResult::Ok | MessageDialogResult::Yes
    )
}
