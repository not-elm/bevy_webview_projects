//! Provides the mechanism to control the window from the webview.

mod center;
mod create;
mod focus;
mod hide;
mod inner_position;
mod inner_size;
mod is_decorated;
mod is_focused;
mod is_fullscreen;
mod is_maximizable;
mod is_maximized;
mod is_minimizable;
mod is_minimized;
mod is_resizable;
mod is_visible;
mod maximize;
mod minimize;
mod outer_position;
mod set_cursor_hit_test;
mod set_decorations;
mod set_position;
mod set_window_mode;
mod show;
mod title;
mod un_focus;
mod un_maximize;
mod un_minimize;

pub use crate::web_window::center::WebWindowCenterPlugin;
pub use crate::web_window::create::WebWindowCreatePlugin;
pub use crate::web_window::focus::WebWindowFocusPlugin;
pub use crate::web_window::hide::WebWindowHidePlugin;
pub use crate::web_window::inner_position::WebWindowInnerPositionPlugin;
pub use crate::web_window::inner_size::WebWindowInnerSizePlugin;
pub use crate::web_window::is_decorated::WebWindowIsDecoratedPlugin;
pub use crate::web_window::is_focused::WebWindowIsFocusedPlugin;
pub use crate::web_window::is_fullscreen::WebWindowIsFullscreenPlugin;
pub use crate::web_window::is_maximizable::WebWindowIsMaximizablePlugin;
pub use crate::web_window::is_maximized::WebWindowIsMaximizedPlugin;
pub use crate::web_window::is_minimizable::WebWindowIsMinimizablePlugin;
pub use crate::web_window::is_minimized::WebWindowIsMinimizedPlugin;
pub use crate::web_window::is_resizable::WebWindowIsResizablePlugin;
pub use crate::web_window::is_visible::WebWindowIsVisiblePlugin;
pub use crate::web_window::maximize::WebWindowMaximizePlugin;
pub use crate::web_window::minimize::WebWindowMinimizePlugin;
pub use crate::web_window::outer_position::WebWindowOuterPositionPlugin;
pub use crate::web_window::set_cursor_hit_test::WebWindowSetCursorHitTestPlugin;
pub use crate::web_window::set_decorations::WebWindowSetDecorationsPlugin;
pub use crate::web_window::set_position::WebWindowSetPositionPlugin;
pub use crate::web_window::set_window_mode::WebWindowSetWindowModePlugin;
pub use crate::web_window::show::WebWindowShowPlugin;
pub use crate::web_window::title::WebWindowTitlePlugin;
pub use crate::web_window::un_focus::WebWindowUnFocusPlugin;
pub use crate::web_window::un_maximize::WebWindowUnMaximizePlugin;
pub use crate::web_window::un_minimize::WebWindowUnMinimizePlugin;
use bevy::app::PluginGroupBuilder;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, Mut, Name, NonSend, PluginGroup, Query, Window};
use bevy::window::WindowWrapper;
use bevy::winit::WinitWindows;
use bevy_webview_core::prelude::EmbedWithin;

/// Allows you to use all window plugins.
///
///## Plugins
///
/// - [WebWindowTitlePlugin]
/// - [WebWindowCenterPlugin]
/// - [WebWindowInnerPositionPlugin]
/// - [WebWindowOuterPositionPlugin]
/// - [WebWindowSetPositionPlugin]
/// - [WebWindowHidePlugin]
/// - [WebWindowShowPlugin]
/// - [WebWindowInnerSizePlugin]
/// - [WebWindowIsDecoratedPlugin]
/// - [WebWindowIsFocusedPlugin]
/// - [WebWindowIsFullscreenPlugin]
/// - [WebWindowIsMaximizedPlugin]
/// - [WebWindowIsMaximizablePlugin]
/// - [WebWindowIsMinimizablePlugin]
/// - [WebWindowIsMinimizedPlugin]
/// - [WebWindowIsResizablePlugin]
/// - [WebWindowIsVisiblePlugin]
/// - [WebWindowMaximizePlugin]
/// - [WebWindowUnMaximizePlugin]
/// - [WebWindowMinimizePlugin]
/// - [WebWindowUnMinimizePlugin]
/// - [WebWindowSetDecorationsPlugin]
/// - [WebWindowSetWindowModePlugin]
/// - [WebWindowFocusPlugin]
/// - [WebWindowUnFocusPlugin]
/// - [WebWindowSetCursorHitTestPlugin]
/// - [WebWindowCreatePlugin]
pub struct AllWebWindowPlugins;
impl PluginGroup for AllWebWindowPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WebWindowTitlePlugin)
            .add(WebWindowCenterPlugin)
            .add(WebWindowInnerPositionPlugin)
            .add(WebWindowOuterPositionPlugin)
            .add(WebWindowSetPositionPlugin)
            .add(WebWindowHidePlugin)
            .add(WebWindowShowPlugin)
            .add(WebWindowInnerSizePlugin)
            .add(WebWindowIsDecoratedPlugin)
            .add(WebWindowIsFocusedPlugin)
            .add(WebWindowIsFullscreenPlugin)
            .add(WebWindowIsMaximizedPlugin)
            .add(WebWindowIsMaximizablePlugin)
            .add(WebWindowIsMinimizablePlugin)
            .add(WebWindowIsMinimizedPlugin)
            .add(WebWindowIsResizablePlugin)
            .add(WebWindowIsVisiblePlugin)
            .add(WebWindowMaximizePlugin)
            .add(WebWindowUnMaximizePlugin)
            .add(WebWindowMinimizePlugin)
            .add(WebWindowUnMinimizePlugin)
            .add(WebWindowSetDecorationsPlugin)
            .add(WebWindowSetWindowModePlugin)
            .add(WebWindowFocusPlugin)
            .add(WebWindowUnFocusPlugin)
            .add(WebWindowSetCursorHitTestPlugin)
            .add(WebWindowCreatePlugin)
    }
}
#[derive(SystemParam)]
struct WebWinitWindowParams<'w, 's> {
    views: Query<
        'w,
        's,
        (
            Entity,
            Option<&'static Name>,
            Option<&'static mut Window>,
            Option<&'static EmbedWithin>,
        ),
    >,
    windows: NonSend<'w, WinitWindows>,
}

impl WebWinitWindowParams<'_, '_> {
    fn bevy_window_mut(&mut self, identifier: &str) -> Option<Mut<Window>> {
        let entity = self.entity(identifier)?;
        self.views.get_mut(entity).ok().and_then(|query| query.2)
    }

    fn winit_window(&self, identifier: &str) -> Option<&WindowWrapper<winit::window::Window>> {
        let entity = self.entity(identifier)?;
        self.windows.get_window(entity)
    }

    fn entity(&self, identifier: &str) -> Option<Entity> {
        self.views.iter().find_map(|(entity, name, _, parent)| {
            if name.as_ref().map(|n| n.as_str()).unwrap_or_default() != identifier {
                return None;
            }
            if let Some(parent) = parent {
                Some(parent.0)
            } else {
                Some(entity)
            }
        })
    }
}
