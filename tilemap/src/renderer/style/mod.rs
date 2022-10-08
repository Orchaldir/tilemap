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
pub struct Style {
    floors: ResourceManager<FloorStyle>,
    solids: ResourceManager<SolidStyle>,
    walls: ResourceManager<WallStyle>,
    grid: Color,
}

impl Style {
    /// Many styles per type.
    pub fn new(
        floors: ResourceManager<FloorStyle>,
        solids: ResourceManager<SolidStyle>,
        walls: ResourceManager<WallStyle>,
        grid: Color,
    ) -> Self {
        Style {
            floors,
            solids,
            walls,
            grid,
        }
    }

    pub fn without_manager(
        floors: Vec<FloorStyle>,
        solids: Vec<SolidStyle>,
        walls: Vec<WallStyle>,
        grid: Color,
    ) -> Self {
        Style {
            floors: ResourceManager::with_default(floors),
            solids: ResourceManager::with_default(solids),
            walls: ResourceManager::with_default(walls),
            grid,
        }
    }

    /// Only one style per type.
    pub fn one_style(floor: FloorStyle, solid: SolidStyle, wall: WallStyle, grid: Color) -> Self {
        Style {
            floors: ResourceManager::new(Vec::new(), floor),
            solids: ResourceManager::new(Vec::new(), solid),
            walls: ResourceManager::new(Vec::new(), wall),
            grid,
        }
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

    pub fn get_grid_color(&self) -> &Color {
        &self.grid
    }
}
