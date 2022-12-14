use crate::math::color::PINK;
use crate::renderer::style::aab::BoxStyle;
use crate::tilemap::NodeId;
use crate::utils::resource::Resource;

/// Defines how to render a [`wall`](crate::tilemap::border::Border::Wall).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WallStyle {
    name: String,
    aab: BoxStyle,
    node: NodeId,
    thickness: u32,
}

impl WallStyle {
    pub fn new<S: Into<String>>(name: S, aab: BoxStyle, node: NodeId, thickness: u32) -> Self {
        WallStyle {
            name: name.into(),
            aab,
            node,
            thickness,
        }
    }

    pub fn get_aab_style(&self) -> &BoxStyle {
        &self.aab
    }

    pub fn get_node_style(&self) -> NodeId {
        self.node
    }

    pub fn get_thickness(&self) -> u32 {
        self.thickness
    }
}

impl Default for WallStyle {
    fn default() -> Self {
        Self::new("default", BoxStyle::shaded(PINK), 0, 1)
    }
}

impl Resource for WallStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
