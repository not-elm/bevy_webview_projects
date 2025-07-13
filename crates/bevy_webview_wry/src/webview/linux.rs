use crate::prelude::WryWebViews;
use bevy::app::{App, First};
use bevy::prelude::{NonSend, Plugin};

pub struct WebviewSupportLinuxPlugin;

impl Plugin for WebviewSupportLinuxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, main_iteration_do);
        gtk::init().expect("Failed to initialize GTK.");
    }
}

fn main_iteration_do(_: NonSend<WryWebViews>) {
    gtk::main_iteration_do(false);
}
