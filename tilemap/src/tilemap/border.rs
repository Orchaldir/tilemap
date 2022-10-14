use crate::math::size2d::Size2d;
use crate::tilemap::style::{DoorId, WallId};

/// The border between 2 [`tiles`](crate::tilemap::tile::Tile).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Border {
    /// No border between the 2 tiles.
    NoBorder,
    /// A wall blocks the border between the 2 tiles.
    Wall(WallId),
    /// A wall blocks the border between the 2 tiles.
    Door(WallId, DoorId),
}

impl Border {
    pub fn get_wall_style(&self) -> Option<WallId> {
        match self {
            Border::NoBorder => None,
            Border::Wall(id) => Some(*id),
            Border::Door(id, _) => Some(*id),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use Border::*;

    const WALL: Border = Wall(42);
    const DOOR: Border = Door(101, 102);

    #[test]
    fn test_get_wall_style() {
        assert_eq!(NoBorder.get_wall_style(), None);
        assert_eq!(WALL.get_wall_style(), Some(42));
        assert_eq!(DOOR.get_wall_style(), Some(101));
    }
}
