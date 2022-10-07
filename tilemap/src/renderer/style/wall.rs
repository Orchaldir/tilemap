use crate::renderer::style::aab::BoxStyle;

/// Defines how to render a [`wall`](crate::tilemap::border::Border::Wall).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WallStyle {
    name: String,
    aab: BoxStyle,
    thickness: u32,
}

impl WallStyle {
    pub fn new<S: Into<String>>(name: S, aab: BoxStyle, thickness: u32) -> Self {
        WallStyle {
            name: name.into(),
            aab,
            thickness,
        }
    }

    pub fn get_aab_style(&self) -> &BoxStyle {
        &self.aab
    }

    pub fn get_thickness(&self) -> u32 {
        self.thickness
    }
}
