use crate::bundle::embedding::resize::ResizeMode;
use bevy::prelude::{Component, ReflectComponent};
use bevy::prelude::{Rect, Vec2};
use bevy::prelude::{Reflect, ReflectDefault, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

/// Represents the display area of a webview within the parent [`Window`](bevy::prelude::Window).
///
/// All data in the field is represented by logical pixels.
#[derive(Component, PartialEq, Reflect, Default, Debug, Copy, Clone, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct Bounds {
    /// Webview size
    pub size: Vec2,

    /// Minimum webview size
    pub min_size: Vec2,

    /// Position of the webview with respect to the top left.
    pub position: Vec2,
}

impl Bounds {
    // noinspection DuplicatedCode
    /// Returns the resize direction of the webview if the cursor is within the resize area.
    #[inline(always)]
    pub fn maybe_resizable(
        &self,
        cursor_pos: Vec2,
        toolbar_height: Option<f32>,
    ) -> Option<ResizeMode> {
        const MARGIN_VEC: Vec2 = Vec2::splat(5.);
        let tool = Vec2::new(0., toolbar_height.unwrap_or(0.));
        let o = self.position - tool;
        let s = self.size + tool;
        let rect = Rect::new(o.x, o.y, o.x + s.x, o.y + s.y);

        if Rect::from_center_size(
            rect.center(),
            (rect.size() - Vec2::splat(0.01)).max(self.min_size),
        )
        .contains(cursor_pos)
        {
            return None;
        }
        if !Rect::from_center_size(rect.center(), rect.size() + 2. * MARGIN_VEC)
            .contains(cursor_pos)
        {
            return None;
        }

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

    /// Transforms the webview size and position based on the resize direction.
    #[inline]
    pub fn transform(&mut self, mode: &ResizeMode, mouse_position: Vec2, toolbar_height: f32) {
        let min_size = self.min_size.max(Vec2::ZERO);
        match mode {
            ResizeMode::Left => {
                let e_x = self.position.x + self.size.x;
                self.size.x = min_size
                    .x
                    .max((self.position.x + self.size.x) - mouse_position.x);
                self.position.x = e_x - self.size.x;
            }
            ResizeMode::Right => {
                self.size.x = min_size.x.max(mouse_position.x - self.position.x);
            }
            ResizeMode::Top => {
                let e_y = self.position.y + self.size.y;
                self.size.y = min_size.y.max(e_y - mouse_position.y - toolbar_height);
                self.position.y = e_y - self.size.y;
            }
            ResizeMode::Bottom => {
                self.size.y = min_size.y.max(mouse_position.y - self.position.y);
            }
            ResizeMode::TopLeft => {
                self.transform(&ResizeMode::Top, mouse_position, toolbar_height);
                self.transform(&ResizeMode::Left, mouse_position, toolbar_height);
            }
            ResizeMode::BottomLeft => {
                self.transform(&ResizeMode::Bottom, mouse_position, toolbar_height);
                self.transform(&ResizeMode::Left, mouse_position, toolbar_height);
            }
            ResizeMode::TopRight => {
                self.transform(&ResizeMode::Top, mouse_position, toolbar_height);
                self.transform(&ResizeMode::Right, mouse_position, toolbar_height);
            }
            ResizeMode::BottomRight => {
                self.transform(&ResizeMode::Bottom, mouse_position, toolbar_height);
                self.transform(&ResizeMode::Right, mouse_position, toolbar_height);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Bounds, ResizeMode};
    use bevy::prelude::*;

    #[test]
    fn expand_from_left() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        bounds.transform(&ResizeMode::Left, Vec2::new(0., 0.), 0.);
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
        bounds.transform(&ResizeMode::Left, Vec2::new(7., 0.), 0.);
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
        bounds.transform(&ResizeMode::Right, Vec2::new(20., 0.), 0.);
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
        bounds.transform(&ResizeMode::Right, Vec2::new(8., 0.), 0.);
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
        bounds.transform(&ResizeMode::Top, Vec2::new(3., 0.), 0.);
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
        bounds.transform(&ResizeMode::Top, Vec2::new(8., 8.), 0.);
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
        bounds.transform(&ResizeMode::Bottom, Vec2::new(8., 15.), 0.);
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
        bounds.transform(&ResizeMode::Bottom, Vec2::new(8., 8.), 0.);
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
        bounds.transform(&ResizeMode::TopLeft, Vec2::new(0., 0.), 0.);
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
        bounds.transform(&ResizeMode::TopLeft, Vec2::new(6., 7.), 0.);
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
        bounds.transform(&ResizeMode::BottomLeft, Vec2::new(0., 15.), 0.);
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
        bounds.transform(&ResizeMode::BottomLeft, Vec2::new(6., 6.), 0.);
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
        bounds.transform(&ResizeMode::TopRight, Vec2::new(10., 0.), 0.);
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
        bounds.transform(&ResizeMode::TopRight, Vec2::new(7., 8.), 0.);
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
        bounds.transform(&ResizeMode::BottomRight, Vec2::new(10., 15.), 0.);
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
        bounds.transform(&ResizeMode::BottomRight, Vec2::new(6., 8.), 0.);
        assert_eq!(bounds.position, Vec2::new(5., 5.));
        assert_eq!(bounds.size, Vec2::new(1., 3.));
    }

    #[test]
    fn min_size_left() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 5.),
            min_size: Vec2::new(2., 1.),
        };
        bounds.transform(&ResizeMode::Left, Vec2::new(7., 0.), 0.);
        assert_eq!(bounds.position, Vec2::new(6., 5.));
        assert_eq!(bounds.size, Vec2::new(2., 5.));
    }

    #[test]
    fn min_size_right() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 5.),
            min_size: Vec2::new(2., 1.),
        };
        bounds.transform(&ResizeMode::Right, Vec2::new(1., 0.), 0.);
        assert_eq!(bounds.position, Vec2::new(5., 5.));
        assert_eq!(bounds.size, Vec2::new(2., 5.));
    }

    #[test]
    fn min_size_top() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 3.),
            min_size: Vec2::new(2., 1.),
        };
        bounds.transform(&ResizeMode::Top, Vec2::new(1., 8.), 0.);
        assert_eq!(bounds.position, Vec2::new(5., 7.));
        assert_eq!(bounds.size, Vec2::new(3., 1.));
    }

    #[test]
    fn min_size_bottom() {
        let mut bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(3., 3.),
            min_size: Vec2::new(2., 1.),
        };
        bounds.transform(&ResizeMode::Bottom, Vec2::new(1., 0.), 0.);
        assert_eq!(bounds.position, Vec2::new(5., 5.));
        assert_eq!(bounds.size, Vec2::new(3., 1.));
    }

    #[test]
    fn resize_top() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(
            bounds.maybe_resizable(Vec2::new(6., 3.), None),
            Some(ResizeMode::Top)
        );
    }

    #[test]
    fn resize_top_left() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(
            bounds.maybe_resizable(Vec2::new(4.5, 4.5), None),
            Some(ResizeMode::TopLeft)
        );
    }

    #[test]
    fn resize_left() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(
            bounds.maybe_resizable(Vec2::new(4.5, 6.), None),
            Some(ResizeMode::Left)
        );
    }

    #[test]
    fn resize_bottom_left() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(
            bounds.maybe_resizable(Vec2::new(4.5, 11.), None),
            Some(ResizeMode::BottomLeft)
        );
    }

    #[test]
    fn resize_bottom() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(
            bounds.maybe_resizable(Vec2::new(7.5, 11.), None),
            Some(ResizeMode::Bottom)
        );
    }

    #[test]
    fn resize_right() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(
            bounds.maybe_resizable(Vec2::new(13.5, 8.), None),
            Some(ResizeMode::Right)
        );
    }

    #[test]
    fn resize_right_up() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(
            bounds.maybe_resizable(Vec2::new(11., 3.), None),
            Some(ResizeMode::TopRight)
        );
    }

    #[test]
    fn resize_top_with_toolbar() {
        let bounds = Bounds {
            position: Vec2::new(100., 100.),
            size: Vec2::new(100., 100.),
            ..default()
        };
        assert_eq!(
            bounds.maybe_resizable(Vec2::new(150., 94.), Some(5.)),
            Some(ResizeMode::Top)
        );
        assert_eq!(bounds.maybe_resizable(Vec2::new(150., 96.), Some(5.)), None);
    }
}
