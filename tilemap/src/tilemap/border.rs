use crate::math::size2d::Size2d;

/// The border between 2 [`tiles`](crate::tilemap::tile::Tile).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Border {
    /// No border between the 2 tiles.
    Empty,
    /// A wall blocks the border between the 2 tiles.
    Wall,
}

/// Returns the size of the horizontal [`borders`](Border) based on the size of the [`tilemap`](crate::tilemap::tilemap2d::Tilemap2d).
pub fn get_horizontal_borders_size(size: Size2d) -> Size2d {
    Size2d::new(size.width(), size.height() + 1)
}

/// Returns the size of the vertical [`borders`](Border) based on the size of the [`tilemap`](crate::tilemap::tilemap2d::Tilemap2d).
pub fn get_vertical_borders_size(size: Size2d) -> Size2d {
    Size2d::new(size.width() + 1, size.height())
}
