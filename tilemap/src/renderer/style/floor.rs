use crate::math::color::{Color, PINK};
use crate::utils::resource::Resource;

/// Defines how to render a [`floor tile`](crate::tilemap::tile::Tile::Floor).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloorStyle {
    name: String,
    floor: Color,
}

impl FloorStyle {
    pub fn new<S: Into<String>>(name: S, floor: Color) -> Self {
        FloorStyle {
            name: name.into(),
            floor,
        }
    }

    pub fn get_floor_color(&self) -> &Color {
        &self.floor
    }
}

impl Default for FloorStyle {
    fn default() -> Self {
        Self::new("default", PINK)
    }
}

impl Resource for FloorStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
