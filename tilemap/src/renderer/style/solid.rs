use crate::renderer::style::aab::BoxStyle;

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
