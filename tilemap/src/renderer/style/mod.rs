use crate::math::color::Color;

mod floor;
mod solid;
mod wall;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Style {
    Simple {
        floor: Color,
        front: Color,
        grid: Color,
        side: Color,
        top: Color,
        wall_thickness: u32,
    },
}

impl Style {
    pub fn new_simple(
        floor: Color,
        front: Color,
        grid: Color,
        side: Color,
        top: Color,
        wall_thickness: u32,
    ) -> Self {
        Style::Simple {
            floor,
            front,
            grid,
            side,
            top,
            wall_thickness,
        }
    }

    pub fn get_floor_color(&self) -> &Color {
        match self {
            Style::Simple { floor, .. } => floor,
        }
    }

    pub fn get_front_color(&self) -> &Color {
        match self {
            Style::Simple { front, .. } => front,
        }
    }

    pub fn get_grid_color(&self) -> &Color {
        match self {
            Style::Simple { grid, .. } => grid,
        }
    }

    pub fn get_side_color(&self) -> &Color {
        match self {
            Style::Simple { side, .. } => side,
        }
    }

    pub fn get_top_color(&self) -> &Color {
        match self {
            Style::Simple { top, .. } => top,
        }
    }

    pub fn get_wall_thickness(&self) -> u32 {
        match self {
            Style::Simple { wall_thickness, .. } => *wall_thickness,
        }
    }
}
