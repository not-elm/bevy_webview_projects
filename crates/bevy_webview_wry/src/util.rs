use bevy::log::error;
use bevy_webview_core::prelude::Bounds;

pub(crate) trait WryResultLog {
    fn output_log_if_failed(self);
}

impl<T> WryResultLog for wry::Result<T> {
    fn output_log_if_failed(self) {
        if let Err(e) = self {
            error!("{e}");
        }
    }
}

#[inline]
pub(crate) fn as_wry_rect(bounds: &Bounds) -> wry::Rect {
    wry::Rect {
        position: wry::dpi::LogicalPosition::new(bounds.position.x, bounds.position.y).into(),
        size: wry::dpi::LogicalSize::new(bounds.size.x, bounds.size.y).into(),
    }
}
