use crate::math::color::Color;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Style {
    Simple {
        floor: Color,
        front: Color,
        side: Color,
        top: Color,
    },
}

impl Style {
    pub fn new_simple(floor: Color, front: Color, side: Color, top: Color) -> Self {
        Style::Simple {
            floor,
            front,
            side,
            top,
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
}
