use bevy::math::{Rect, Vec2};
use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};

use crate::as_child::bundle::resize::ResizeMode;

#[derive(Component, PartialEq, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Bounds {
    pub size: Vec2,

    pub min_size: Vec2,

    pub position: Vec2,
}

impl Bounds {
    // noinspection DuplicatedCode
    #[inline(always)]
    pub(crate) fn maybe_resizable(&self, cursor_pos: Vec2) -> Option<ResizeMode> {
        const MARGIN_VEC: Vec2 = Vec2::splat(10.);

        let o = self.position - MARGIN_VEC;
        let s = self.size + 2. * MARGIN_VEC;
        if !Rect::new(o.x, o.y, o.x + s.x, o.y + s.y).contains(cursor_pos) {
            return None;
        }
        let o = self.position;
        let s = self.size;
        let rect = Rect::new(o.x, o.y, o.x + s.x, o.y + s.y);

        if cursor_pos.x <= rect.min.x {
            return if cursor_pos.y <= rect.min.y {
                Some(ResizeMode::TopLeft)
            } else if rect.max.y <= cursor_pos.y {
                Some(ResizeMode::BottomLeft)
            } else {
                Some(ResizeMode::Left)
            };
        }
        if rect.max.x <= cursor_pos.x {
            return if cursor_pos.y <= rect.min.y {
                Some(ResizeMode::TopRight)
            } else if rect.max.y <= cursor_pos.y {
                Some(ResizeMode::BottomRight)
            } else {
                Some(ResizeMode::Right)
            };
        }
        if cursor_pos.y <= rect.min.y {
            Some(ResizeMode::Top)
        } else {
            Some(ResizeMode::Bottom)
        }
    }

    #[inline]
    pub(crate) fn resize(&mut self, mode: &ResizeMode, mouse_position: Vec2) {
        match mode {
            ResizeMode::Left => {
                self.size.x = (self.position.x + self.size.x) - mouse_position.x;
                self.position.x = mouse_position.x;
            }
            ResizeMode::Right => {
                self.size.x = mouse_position.x - self.position.x;
            }
            ResizeMode::Top => {
                self.size.y = (self.position.y + self.size.y) - mouse_position.y;
                self.position.y = mouse_position.y;
            }
            ResizeMode::Bottom => {
                self.size.y = mouse_position.y - self.position.y;
            }
            ResizeMode::TopLeft => {
                self.resize(&ResizeMode::Top, mouse_position);
                self.resize(&ResizeMode::Left, mouse_position);
            }
            ResizeMode::BottomLeft => {
                self.resize(&ResizeMode::Bottom, mouse_position);
                self.resize(&ResizeMode::Left, mouse_position);
            }
            ResizeMode::TopRight => {
                self.resize(&ResizeMode::Top, mouse_position);
                self.resize(&ResizeMode::Right, mouse_position);
            }
            ResizeMode::BottomRight => {
                self.resize(&ResizeMode::Bottom, mouse_position);
                self.resize(&ResizeMode::Right, mouse_position);
            }
        }
    }

    pub(crate) fn as_wry_rect(&self) -> wry::Rect {
        wry::Rect {
            position: wry::dpi::LogicalPosition::new(self.position.x, self.position.y).into(),
            size: wry::dpi::LogicalSize::new(self.size.x, self.size.y).into(),
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use bevy::utils::default;

    use crate::as_child::bundle::Bounds;
    use crate::prelude::resize::ResizeMode;

    #[test]
    fn expand_from_left() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::Left, Vec2::new(0., 0.));
        assert_eq!(bounds.position, Vec2::new(0., 5.));
        assert_eq!(bounds.size, Vec2::new(10., 5.));
    }

    #[test]
    fn shrink_from_left() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::Left, Vec2::new(7., 0.));
        assert_eq!(bounds.position, Vec2::new(7., 5.));
        assert_eq!(bounds.size, Vec2::new(3., 5.));
    }

    #[test]
    fn expand_from_right() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::Right, Vec2::new(20., 0.));
        assert_eq!(bounds.position, Vec2::new(5., 5.));
        assert_eq!(bounds.size, Vec2::new(15., 5.));
    }

    #[test]
    fn shrink_from_right() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::Right, Vec2::new(8., 0.));
        assert_eq!(bounds.position, Vec2::new(5., 5.));
        assert_eq!(bounds.size, Vec2::new(3., 5.));
    }

    #[test]
    fn expand_from_top() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::Top, Vec2::new(3., 0.));
        assert_eq!(bounds.position, Vec2::new(5., 0.));
        assert_eq!(bounds.size, Vec2::new(5., 10.));
    }

    #[test]
    fn shrink_from_top() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::Top, Vec2::new(8., 8.));
        assert_eq!(bounds.position, Vec2::new(5., 8.));
        assert_eq!(bounds.size, Vec2::new(5., 2.));
    }

    #[test]
    fn expand_from_bottom() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::Bottom, Vec2::new(8., 15.));
        assert_eq!(bounds.position, Vec2::new(5., 5.));
        assert_eq!(bounds.size, Vec2::new(5., 10.));
    }

    #[test]
    fn shrink_from_bottom() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::Bottom, Vec2::new(8., 8.));
        assert_eq!(bounds.position, Vec2::new(5., 5.));
        assert_eq!(bounds.size, Vec2::new(5., 3.));
    }

    #[test]
    fn expand_from_top_left() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::TopLeft, Vec2::new(0., 0.));
        assert_eq!(bounds.position, Vec2::new(0., 0.));
        assert_eq!(bounds.size, Vec2::new(8., 10.));
    }

    #[test]
    fn shrink_from_top_left() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::TopLeft, Vec2::new(6., 7.));
        assert_eq!(bounds.position, Vec2::new(6., 7.));
        assert_eq!(bounds.size, Vec2::new(2., 3.));
    }

    #[test]
    fn expand_from_bottom_left() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::BottomLeft, Vec2::new(0., 15.));
        assert_eq!(bounds.position, Vec2::new(0., 5.));
        assert_eq!(bounds.size, Vec2::new(8., 10.));
    }

    #[test]
    fn shrink_from_bottom_left() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::BottomLeft, Vec2::new(6., 6.));
        assert_eq!(bounds.position, Vec2::new(6., 5.));
        assert_eq!(bounds.size, Vec2::new(2., 1.));
    }

    #[test]
    fn expand_from_top_right() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::TopRight, Vec2::new(10., 0.));
        assert_eq!(bounds.position, Vec2::new(5., 0.));
        assert_eq!(bounds.size, Vec2::new(5., 10.));
    }

    #[test]
    fn shrink_from_top_right() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::TopRight, Vec2::new(7., 8.));
        assert_eq!(bounds.position, Vec2::new(5., 8.));
        assert_eq!(bounds.size, Vec2::new(2., 2.));
    }

    #[test]
    fn expand_from_bottom_right() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::BottomRight, Vec2::new(10., 15.));
        assert_eq!(bounds.position, Vec2::new(5., 5.));
        assert_eq!(bounds.size, Vec2::new(5., 10.));
    }

    #[test]
    fn shrink_from_bottom_right() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 5.),
            ..default()
        };
        bounds.resize(&ResizeMode::BottomRight, Vec2::new(6., 8.));
        assert_eq!(bounds.position, Vec2::new(5., 5.));
        assert_eq!(bounds.size, Vec2::new(1., 3.));
    }
}
