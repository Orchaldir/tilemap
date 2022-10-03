use crate::math::size2d::Size2d;

/// The border between 2 [`tiles`](crate::tilemap::tile::Tile).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Border {
    /// No border between the 2 tiles.
    Empty,
    /// A wall blocks the border between the 2 tiles.
    Wall(usize),
}

/// Returns the size of the horizontal [`borders`](Border) based on the size of the [`tilemap`](crate::tilemap::tilemap2d::Tilemap2d).
pub fn get_horizontal_borders_size(size: Size2d) -> Size2d {
    Size2d::new(size.width(), size.height() + 1)
}

/// Returns the size of the vertical [`borders`](Border) based on the size of the [`tilemap`](crate::tilemap::tilemap2d::Tilemap2d).
pub fn get_vertical_borders_size(size: Size2d) -> Size2d {
    Size2d::new(size.width() + 1, size.height())
}

/// Returns the index of the horizontal [`Border`] behind the [`tile`](crate::tilemap::tile::Tile).
pub fn behind_tile(_size: Size2d, tile_index: usize) -> usize {
    tile_index
}

/// Returns the index of the horizontal [`Border`] left of the [`tile`](crate::tilemap::tile::Tile).
pub fn left_of_tile(size: Size2d, tile_index: usize) -> usize {
    tile_index + size.to_y(tile_index) as usize
}

/// Returns the index of the horizontal [`Border`] in front the [`tile`](crate::tilemap::tile::Tile).
pub fn in_front_of_tile(size: Size2d, tile_index: usize) -> usize {
    tile_index + size.width() as usize
}

/// Returns the index of the vertical [`Border`] to the right of the [`tile`](crate::tilemap::tile::Tile).
pub fn right_of_tile(size: Size2d, tile_index: usize) -> usize {
    left_of_tile(size, tile_index) + 1
}
