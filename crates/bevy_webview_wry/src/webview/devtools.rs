use crate::webview::WryWebViews;
use bevy::prelude::{
    App, Commands, Component, DetectChanges, Entity, NonSend, Plugin, PostUpdate, Query, Reflect,
    ReflectComponent,
};
use bevy_webview_core::prelude::{IsOpenDevtools, UseDevtools};

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DevtoolsReady>()
            .add_systems(PostUpdate, change_open_devtools);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct DevtoolsReady;

fn change_open_devtools(
    mut commands: Commands,
    mut views: Query<(
        Entity,
        &mut IsOpenDevtools,
        &UseDevtools,
        Option<&DevtoolsReady>,
    )>,
    web_views: NonSend<WryWebViews>,
) {
    for (entity, mut is_open, use_devtools, ready) in views.iter_mut() {
        if !use_devtools.0 {
            is_open.0 = false;
            continue;
        }
        let Some(webview) = web_views.0.get(&entity) else {
            continue;
        };
        if ready.is_none() || is_open.is_changed() {
            if is_open.0 {
                webview.open_devtools();
            } else {
                webview.close_devtools();
            }
            commands.entity(entity).insert(DevtoolsReady);
        } else {
            is_open.0 = webview.is_devtools_open();
        }
    }
}
