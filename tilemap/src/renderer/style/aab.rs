use crate::math::color::Color;

/// Defines how to render an axis aligned box.
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
