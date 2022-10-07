use crate::math::color::Color;
use crate::renderer::style::floor::FloorStyle;
use crate::renderer::style::solid::SolidStyle;
use crate::renderer::style::wall::WallStyle;

pub mod aab;
pub mod floor;
pub mod solid;
pub mod wall;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Style {
    Simple {
        floor: FloorStyle,
        solid: SolidStyle,
        wall: WallStyle,
        grid: Color,
    },
}

impl Style {
    pub fn new_simple(floor: FloorStyle, solid: SolidStyle, wall: WallStyle, grid: Color) -> Self {
        Style::Simple {
            floor,
            solid,
            wall,
            grid,
        }
    }

    pub fn get_floor_style(&self) -> &FloorStyle {
        match self {
            Style::Simple { floor, .. } => floor,
        }
    }

    pub fn get_solid_style(&self) -> &SolidStyle {
        match self {
            Style::Simple { solid, .. } => solid,
        }
    }

    pub fn get_wall_style(&self) -> &WallStyle {
        match self {
            Style::Simple { wall, .. } => wall,
        }
    }

    pub fn get_grid_color(&self) -> &Color {
        match self {
            Style::Simple { grid, .. } => grid,
        }
    }
}
