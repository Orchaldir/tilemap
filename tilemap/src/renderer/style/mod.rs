use crate::math::color::Color;
use crate::renderer::style::floor::FloorStyle;
use crate::renderer::style::solid::SolidStyle;
use crate::renderer::style::wall::WallStyle;
use crate::tilemap::border::WallId;
use crate::tilemap::tile::{FloorId, SolidId};
use crate::utils::resource::ResourceManager;

pub mod aab;
pub mod floor;
pub mod solid;
pub mod wall;

#[derive(Debug)]
pub struct StyleMgr {
    floors: ResourceManager<FloorStyle>,
    solids: ResourceManager<SolidStyle>,
    walls: ResourceManager<WallStyle>,
    wall_thickness: u32,
    grid: Color,
}

impl StyleMgr {
    /// Many styles per type.
    pub fn new(
        floors: ResourceManager<FloorStyle>,
        solids: ResourceManager<SolidStyle>,
        walls: ResourceManager<WallStyle>,
        wall_thickness: u32,
        grid: Color,
    ) -> Self {
        StyleMgr {
            floors,
            solids,
            walls,
            wall_thickness,
            grid,
        }
    }

    pub fn without_manager(
        floors: Vec<FloorStyle>,
        solids: Vec<SolidStyle>,
        walls: Vec<WallStyle>,
        wall_thickness: u32,
        grid: Color,
    ) -> Self {
        Self::new(
            ResourceManager::with_default(floors),
            ResourceManager::with_default(solids),
            ResourceManager::with_default(walls),
            wall_thickness,
            grid,
        )
    }

    /// Only one style per type.
    pub fn one_style(
        floor: FloorStyle,
        solid: SolidStyle,
        wall: WallStyle,
        wall_thickness: u32,
        grid: Color,
    ) -> Self {
        Self::new(
            ResourceManager::new(Vec::new(), floor),
            ResourceManager::new(Vec::new(), solid),
            ResourceManager::new(Vec::new(), wall),
            wall_thickness,
            grid,
        )
    }

    pub fn get_floor_style(&self, id: FloorId) -> &FloorStyle {
        self.floors.get(id)
    }

    pub fn get_solid_style(&self, id: SolidId) -> &SolidStyle {
        self.solids.get(id)
    }

    pub fn get_wall_style(&self, id: WallId) -> &WallStyle {
        self.walls.get(id)
    }

    pub fn get_wall_thickness(&self) -> u32 {
        self.wall_thickness
    }

    pub fn get_grid_color(&self) -> &Color {
        &self.grid
    }
}
