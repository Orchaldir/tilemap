use crate::math::color::Color;
use crate::renderer::style::door::DoorStyle;
use crate::renderer::style::floor::FloorStyle;
use crate::renderer::style::node::NodeStyle;
use crate::renderer::style::solid::SolidStyle;
use crate::renderer::style::wall::WallStyle;
use crate::tilemap::style::{DoorId, FloorId, NodeId, SolidId, WallId};
use crate::utils::resource::ResourceManager;

pub mod aab;
pub mod door;
pub mod floor;
pub mod node;
pub mod solid;
pub mod wall;

#[derive(Debug)]
pub struct StyleMgr {
    doors: ResourceManager<DoorStyle>,
    floors: ResourceManager<FloorStyle>,
    nodes: ResourceManager<NodeStyle>,
    solids: ResourceManager<SolidStyle>,
    walls: ResourceManager<WallStyle>,
    grid: Color,
}

impl StyleMgr {
    /// Many styles per type.
    pub fn new(
        doors: ResourceManager<DoorStyle>,
        floors: ResourceManager<FloorStyle>,
        nodes: ResourceManager<NodeStyle>,
        solids: ResourceManager<SolidStyle>,
        walls: ResourceManager<WallStyle>,
        grid: Color,
    ) -> Self {
        StyleMgr {
            doors,
            floors,
            nodes,
            solids,
            walls,
            grid,
        }
    }

    pub fn without_manager(
        doors: Vec<DoorStyle>,
        floors: Vec<FloorStyle>,
        nodes: Vec<NodeStyle>,
        solids: Vec<SolidStyle>,
        walls: Vec<WallStyle>,
        grid: Color,
    ) -> Self {
        Self::new(
            ResourceManager::with_default(doors),
            ResourceManager::with_default(floors),
            ResourceManager::with_default(nodes),
            ResourceManager::with_default(solids),
            ResourceManager::with_default(walls),
            grid,
        )
    }

    /// Only one style per type.
    pub fn one_style(
        door: DoorStyle,
        floor: FloorStyle,
        node: NodeStyle,
        solid: SolidStyle,
        wall: WallStyle,
        grid: Color,
    ) -> Self {
        Self::new(
            ResourceManager::new(Vec::new(), door),
            ResourceManager::new(Vec::new(), floor),
            ResourceManager::new(Vec::new(), node),
            ResourceManager::new(Vec::new(), solid),
            ResourceManager::new(Vec::new(), wall),
            grid,
        )
    }

    pub fn get_door_style(&self, id: DoorId) -> &DoorStyle {
        self.doors.get(id)
    }

    pub fn get_floor_style(&self, id: FloorId) -> &FloorStyle {
        self.floors.get(id)
    }

    pub fn get_node_style(&self, id: NodeId) -> &NodeStyle {
        self.nodes.get(id)
    }

    pub fn get_node_styles(&self) -> &ResourceManager<NodeStyle> {
        &self.nodes
    }

    pub fn get_solid_style(&self, id: SolidId) -> &SolidStyle {
        self.solids.get(id)
    }

    pub fn get_wall_style(&self, id: WallId) -> &WallStyle {
        self.walls.get(id)
    }

    pub fn get_wall_styles(&self) -> &ResourceManager<WallStyle> {
        &self.walls
    }

    pub fn get_grid_color(&self) -> &Color {
        &self.grid
    }
}
