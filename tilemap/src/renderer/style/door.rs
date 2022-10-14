use crate::math::color::PINK;
use crate::renderer::style::aab::BoxStyle;
use crate::utils::resource::Resource;

/// Defines how to render a [`door`](crate::tilemap::border::Border::Door).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DoorStyle {
    name: String,
    style: BoxStyle,
}

impl DoorStyle {
    pub fn new<S: Into<String>>(name: S, style: BoxStyle) -> Self {
        DoorStyle {
            name: name.into(),
            style,
        }
    }

    pub fn get_style(&self) -> &BoxStyle {
        &self.style
    }
}

impl Default for DoorStyle {
    fn default() -> Self {
        Self::new("default", BoxStyle::shaded(PINK))
    }
}

impl Resource for DoorStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
