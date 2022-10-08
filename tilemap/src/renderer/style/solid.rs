use crate::math::color::PINK;
use crate::renderer::style::aab::BoxStyle;
use crate::utils::resource::Resource;

/// Defines how to render a [`solid tile`](crate::tilemap::tile::Tile::Solid).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolidStyle {
    name: String,
    aab: BoxStyle,
}

impl SolidStyle {
    pub fn new<S: Into<String>>(name: S, aab: BoxStyle) -> Self {
        SolidStyle {
            name: name.into(),
            aab,
        }
    }

    pub fn get_aab_style(&self) -> &BoxStyle {
        &self.aab
    }
}

impl Default for SolidStyle {
    fn default() -> Self {
        Self::new("default", BoxStyle::shaded(PINK))
    }
}

impl Resource for SolidStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
