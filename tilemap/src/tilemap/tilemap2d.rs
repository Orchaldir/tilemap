use crate::math::size2d::Size2d;
use crate::tilemap::border::{get_horizontal_borders_size, get_vertical_borders_size, Border};
use crate::tilemap::tile::Tile;
use anyhow::{bail, Result};

#[svgbobdoc::transform]
/// The tilemap contains a 2d grid of [`tiles`](Tile) and the [`borders`](Border) between them.
/// # Diagram
///
/// ```svgbob
///
///   +----------> x-axis
///   |
///   |             back
///   |      +---------------+
///   |      |               |
///   |      |               |
///   | left |     A tile    | right
///   |      |               |
///   |      |               |
///   |      +---------------+
///   |           front
///   v
/// y-axis
///
/// ```
///
#[derive(Debug, Eq, PartialEq)]
pub struct Tilemap2d {
    /// The size of a rectangle of [`Tile`].
    size: Size2d,
    /// A rectangle of [`tiles`](Tile).
    tiles: Vec<Tile>,
    /// The [`borders`](Border) at the back & front of each [`Tile`].
    horizontal_borders: Vec<Border>,
    /// The [`borders`](Border) to the left & right of each [`Tile`].
    vertical_borders: Vec<Border>,
}

impl Tilemap2d {
    /// Returns a tilemap of the desired [`size`](Size2d) with the default [`Tile`].
    pub fn default(size: Size2d, tile: Tile) -> Result<Tilemap2d> {
        let tiles = vec![tile; size.count()];

        Self::new(size, tiles)
    }

    /// Returns a tilemap with the desired [`tiles`](Tile), if the number of tiles match the [`size`](Size2d).
    pub fn new(size: Size2d, tiles: Vec<Tile>) -> Result<Tilemap2d> {
        let horizontal_borders = vec![Border::Empty; get_horizontal_borders_size(size).count()];
        let vertical_borders = vec![Border::Empty; get_vertical_borders_size(size).count()];

        Self::with_borders(size, tiles, horizontal_borders, vertical_borders)
    }

    /// Creates a tilemap from the desired [`tiles`](Tile) & [`borders`](Border).
    pub fn with_borders(
        size: Size2d,
        tiles: Vec<Tile>,
        horizontal_borders: Vec<Border>,
        vertical_borders: Vec<Border>,
    ) -> Result<Tilemap2d> {
        if size.count() == 0 {
            bail!("The tilemap has a size of 0!");
        } else if size.count() != tiles.len() {
            bail!("Size and number of tiles don't match!");
        } else if get_horizontal_borders_size(size).count() != horizontal_borders.len() {
            bail!("Size and number of horizontal borders don't match!");
        } else if get_vertical_borders_size(size).count() != vertical_borders.len() {
            bail!("Size and number of vertical borders don't match!");
        }

        Ok(Tilemap2d {
            size,
            tiles,
            horizontal_borders,
            vertical_borders,
        })
    }

    pub fn get_size(&self) -> Size2d {
        self.size
    }

    // Tiles

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_tile(&self, index: usize) -> Tile {
        self.tiles[index]
    }

    pub fn set_tile(&mut self, index: usize, tile: Tile) {
        self.tiles[index] = tile;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::size2d::Size2d;

    #[test]
    fn test_default() {
        let size = Size2d::new(2, 3);
        let tilemap = Tilemap2d::default(size, Tile::Empty).unwrap();

        assert_eq!(tilemap.get_size(), size);
        assert_eq!(tilemap.get_tiles(), &vec![Tile::Empty; 6]);
    }

    #[test]
    fn test_new() {
        let size = Size2d::new(2, 3);
        let tiles = create_tiles();
        let tilemap = Tilemap2d::new(size, tiles.clone()).unwrap();

        assert_eq!(tilemap.get_size(), size);

        for i in 0..6 {
            assert_eq!(tilemap.get_tile(i), tiles[i]);
        }
    }

    #[test]
    fn test_new_with_invalid_size() {
        assert!(Tilemap2d::new(Size2d::new(0, 3), vec![]).is_err());
        assert!(Tilemap2d::new(Size2d::new(2, 0), vec![]).is_err());
    }

    #[test]
    fn test_new_with_invalid_tiles() {
        assert!(Tilemap2d::new(Size2d::new(2, 3), vec![Tile::Empty]).is_err());
    }

    #[test]
    fn test_set_tile() {
        let size = Size2d::new(2, 3);
        let mut tilemap = Tilemap2d::default(size, Tile::Empty).unwrap();

        tilemap.set_tile(0, Tile::Floor(1));
        tilemap.set_tile(2, Tile::Solid(3));
        tilemap.set_tile(4, Tile::Floor(4));

        assert_eq!(tilemap.get_size(), size);
        assert_eq!(tilemap.get_tiles(), &create_tiles());
    }

    fn create_tiles() -> Vec<Tile> {
        vec![
            Tile::Floor(1),
            Tile::Empty,
            Tile::Solid(3),
            Tile::Empty,
            Tile::Floor(4),
            Tile::Empty,
        ]
    }
}
