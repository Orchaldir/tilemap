use crate::math::color::Color;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Style {
    Simple { floor: Color, solid: Color },
}

impl Style {
    pub fn new_simple(floor: Color, solid: Color) -> Self {
        Style::Simple { floor, solid }
    }

    pub fn get_floor_color(&self) -> &Color {
        match self {
            Style::Simple { floor, .. } => floor,
        }
    }

    pub fn get_solid_color(&self) -> &Color {
        match self {
            Style::Simple { solid, .. } => solid,
        }
    }
}
