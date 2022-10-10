use crate::math::color::PINK;
use crate::renderer::style::aab::BoxStyle;
use crate::utils::resource::Resource;

/// Defines how to render a node, where [`walls`](crate::tilemap::border::Border::Wall) intersect.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeStyle {
    name: String,
    style: BoxStyle,
    size: u32,
    half: u32,
}

impl NodeStyle {
    pub fn new<S: Into<String>>(name: S, style: BoxStyle, size: u32) -> Self {
        NodeStyle {
            name: name.into(),
            style,
            size,
            half: size / 2,
        }
    }

    pub fn get_style(&self) -> &BoxStyle {
        &self.style
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_half(&self) -> u32 {
        self.half
    }
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self::new("default", BoxStyle::shaded(PINK), 1)
    }
}

impl Resource for NodeStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
