use crate::math::color::Color;

/// Defines how to render a [`solid tile`](crate::tilemap::tile::Tile::Solid).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolidStyle {
    name: String,
    front: Color,
    side: Color,
    top: Color,
}

impl SolidStyle {
    pub fn new<S: Into<String>>(name: S, front: Color, side: Color, top: Color) -> Self {
        SolidStyle {
            name: name.into(),
            front,
            side,
            top,
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
