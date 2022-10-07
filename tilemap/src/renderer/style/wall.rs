use crate::math::color::Color;

/// Defines how to render a [`wall`](crate::tilemap::border::Border::Wall).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WallStyle {
    name: String,
    front: Color,
    side: Color,
    top: Color,
    thickness: u32,
}

impl WallStyle {
    pub fn new<S: Into<String>>(
        name: S,
        front: Color,
        side: Color,
        top: Color,
        thickness: u32,
    ) -> Self {
        WallStyle {
            name: name.into(),
            front,
            side,
            top,
            thickness,
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

    pub fn get_thickness(&self) -> u32 {
        self.thickness
    }
}
