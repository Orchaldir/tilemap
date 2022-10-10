use crate::math::color::Color;
use crate::renderer::style::floor::FloorStyle;
use crate::renderer::style::node::NodeStyle;
use crate::renderer::style::solid::SolidStyle;
use crate::renderer::style::wall::WallStyle;
use crate::tilemap::border::WallId;
use crate::tilemap::tile::{FloorId, SolidId};
use crate::tilemap::NodeId;
use crate::utils::resource::ResourceManager;

pub mod aab;
pub mod floor;
pub mod node;
pub mod solid;
pub mod wall;

#[derive(Debug)]
pub struct StyleMgr {
    floors: ResourceManager<FloorStyle>,
    nodes: ResourceManager<NodeStyle>,
    solids: ResourceManager<SolidStyle>,
    walls: ResourceManager<WallStyle>,
    grid: Color,
}

impl StyleMgr {
    /// Many styles per type.
    pub fn new(
        floors: ResourceManager<FloorStyle>,
        nodes: ResourceManager<NodeStyle>,
        solids: ResourceManager<SolidStyle>,
        walls: ResourceManager<WallStyle>,
        grid: Color,
    ) -> Self {
        StyleMgr {
            floors,
            nodes,
            solids,
            walls,
            grid,
        }
    }

    pub fn without_manager(
        floors: Vec<FloorStyle>,
        nodes: Vec<NodeStyle>,
        solids: Vec<SolidStyle>,
        walls: Vec<WallStyle>,
        grid: Color,
    ) -> Self {
        Self::new(
            ResourceManager::with_default(floors),
            ResourceManager::with_default(nodes),
            ResourceManager::with_default(solids),
            ResourceManager::with_default(walls),
            grid,
        )
    }

    /// Only one style per type.
    pub fn one_style(
        floor: FloorStyle,
        node: NodeStyle,
        solid: SolidStyle,
        wall: WallStyle,
        grid: Color,
    ) -> Self {
        Self::new(
            ResourceManager::new(Vec::new(), floor),
            ResourceManager::new(Vec::new(), node),
            ResourceManager::new(Vec::new(), solid),
            ResourceManager::new(Vec::new(), wall),
            grid,
        )
    }

    pub fn get_floor_style(&self, id: FloorId) -> &FloorStyle {
        self.floors.get(id)
    }

    pub fn get_node_style(&self, id: NodeId) -> &NodeStyle {
        self.nodes.get(id)
    }

    pub fn get_solid_style(&self, id: SolidId) -> &SolidStyle {
        self.solids.get(id)
    }

    pub fn get_wall_style(&self, id: WallId) -> &WallStyle {
        self.walls.get(id)
    }

    pub fn get_grid_color(&self) -> &Color {
        &self.grid
    }
}
