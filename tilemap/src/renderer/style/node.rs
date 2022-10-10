use crate::math::color::PINK;
use crate::renderer::style::aab::BoxStyle;
use crate::utils::resource::Resource;

/// Defines how to render a node, where [`walls`](crate::tilemap::border::Border::Wall) intersect.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeStyle {
    name: String,
    style: BoxStyle,
}

impl NodeStyle {
    pub fn new<S: Into<String>>(name: S, style: BoxStyle) -> Self {
        NodeStyle {
            name: name.into(),
            style,
        }
    }

    pub fn get_style_style(&self) -> &BoxStyle {
        &self.style
    }
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self::new("default", BoxStyle::shaded(PINK))
    }
}

impl Resource for NodeStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
