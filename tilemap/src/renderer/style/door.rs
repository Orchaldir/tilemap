use crate::math::color::PINK;
use crate::renderer::style::aab::BoxStyle;
use crate::utils::resource::Resource;

/// Defines how to render a [`door`](crate::tilemap::border::Border::Door).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DoorStyle {
    name: String,
    style: BoxStyle,
    height: u32,
    thickness: u32,
}

impl DoorStyle {
    pub fn new<S: Into<String>>(name: S, style: BoxStyle, height: u32, thickness: u32) -> Self {
        DoorStyle {
            name: name.into(),
            style,
            height,
            thickness,
        }
    }

    pub fn get_style(&self) -> &BoxStyle {
        &self.style
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_thickness(&self) -> u32 {
        self.thickness
    }
}

impl Default for DoorStyle {
    fn default() -> Self {
        Self::new("default", BoxStyle::shaded(PINK), 2000, 50)
    }
}

impl Resource for DoorStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
