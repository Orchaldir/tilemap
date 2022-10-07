use crate::math::color::Color;

/// Defines how to render an axis aligned box. Used by other styles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoxStyle {
    front: Color,
    side: Color,
    top: Color,
}

impl BoxStyle {
    pub fn new(front: Color, side: Color, top: Color) -> Self {
        BoxStyle { front, side, top }
    }

    /// Fakes lighting by darkening the front & side faces.
    pub fn shaded(color: Color) -> Self {
        BoxStyle {
            front: color * 0.8,
            side: color * 0.6,
            top: color,
        }
    }

    pub fn get_front_color(&self) -> &Color {
        &self.front
    }

    pub fn get_side_color(&self) -> &Color {
        &self.side
    }

    pub fn get_top_color(&self) -> &Color {
        &self.top
    }
}
