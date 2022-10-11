use crate::math::side::Side;
use crate::math::size2d::Size2d;
use crate::tilemap::border::{
    behind_tile, get_horizontal_borders_size, get_vertical_borders_size, in_front_of_tile,
    left_of_tile, right_of_tile, Border,
};
use crate::tilemap::node::get_nodes_size;
use crate::tilemap::tile::Tile;
use anyhow::{bail, Result};
use Side::*;

#[svgbobdoc::transform]
/// The tilemap contains a 2d grid of [`tiles`](Tile) and the [`borders`](Border) between them.
#[derive(Debug, Eq, PartialEq)]
pub struct Tilemap2d {
    /// The size of a rectangle of [`tiles`](Tile).
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
        let horizontal_borders = vec![Border::NoBorder; get_horizontal_borders_size(size).count()];
        let vertical_borders = vec![Border::NoBorder; get_vertical_borders_size(size).count()];

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

    /// Borders

    pub fn get_horizontal_borders(&self) -> &Vec<Border> {
        &self.horizontal_borders
    }

    pub fn get_vertical_borders(&self) -> &Vec<Border> {
        &self.vertical_borders
    }

    /// Returns the border on a specific side of a tile.
    pub fn get_border(&self, tile_index: usize, side: Side) -> Border {
        if tile_index >= self.size.count() {
            panic!("get_border(): Tile {} is outside the tilemap!", tile_index);
        }

        match side {
            Back => self.horizontal_borders[behind_tile(self.size, tile_index)],
            Left => self.vertical_borders[left_of_tile(self.size, tile_index)],
            Front => self.horizontal_borders[in_front_of_tile(self.size, tile_index)],
            Right => self.vertical_borders[right_of_tile(self.size, tile_index)],
        }
    }

    /// Sets the border on a specific side of a tile.
    pub fn set_border(&mut self, tile_index: usize, side: Side, border: Border) {
        if tile_index >= self.size.count() {
            panic!("set_border(): Tile {} is outside the tilemap!", tile_index);
        }

        match side {
            Back => self.horizontal_borders[behind_tile(self.size, tile_index)] = border,
            Left => self.vertical_borders[left_of_tile(self.size, tile_index)] = border,
            Front => self.horizontal_borders[in_front_of_tile(self.size, tile_index)] = border,
            Right => self.vertical_borders[right_of_tile(self.size, tile_index)] = border,
        };
    }

    // nodes

    /// Returns the [`border`](Border) on a specific [`side`](Side) of a node.
    pub fn get_border_at_node(&self, node_index: usize, side: Side) -> Border {
        let nodes_size = get_nodes_size(self.size);

        if node_index >= nodes_size.count() {
            panic!(
                "get_border_at_node(): Node {} is outside the tilemap!",
                node_index
            );
        }

        let point = nodes_size.to_point(node_index);

        match side {
            Back => {
                if point.y == 0 {
                    Border::NoBorder
                } else {
                    self.vertical_borders[node_index - nodes_size.width() as usize]
                }
            }
            Left => {
                if point.x == 0 {
                    Border::NoBorder
                } else {
                    self.horizontal_borders[node_index - 1 - point.y as usize]
                }
            }
            Front => {
                if point.y >= (nodes_size.height() - 1) as i32 {
                    Border::NoBorder
                } else {
                    self.vertical_borders[node_index]
                }
            }
            Right => {
                if point.x == (nodes_size.width() - 1) as i32 {
                    Border::NoBorder
                } else {
                    self.horizontal_borders[node_index - point.y as usize]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::size2d::Size2d;
    use Border::{NoBorder, Wall};
    use Tile::{Empty, Floor, Solid};

    #[test]
    fn test_default() {
        let size = Size2d::new(2, 3);
        let tilemap = Tilemap2d::default(size, Empty).unwrap();

        assert_eq!(tilemap.get_size(), size);
        assert_eq!(tilemap.get_tiles(), &vec![Empty; 6]);
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
        assert!(Tilemap2d::new(Size2d::new(2, 3), vec![Empty]).is_err());
    }

    #[test]
    fn test_set_tile() {
        let size = Size2d::new(2, 3);
        let mut tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.set_tile(0, Floor(1));
        tilemap.set_tile(2, Solid(3));
        tilemap.set_tile(4, Floor(4));

        assert_eq!(tilemap.get_size(), size);
        assert_eq!(tilemap.get_tiles(), &create_tiles());
    }

    #[test]
    #[should_panic]
    fn test_get_border_outside_map() {
        let size = Size2d::new(2, 3);
        let tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.get_border(6, Back);
    }

    #[test]
    fn test_set_border() {
        let size = Size2d::new(2, 3);
        let mut tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.set_border(5, Back, Wall(1));
        tilemap.set_border(5, Left, Wall(2));
        tilemap.set_border(5, Front, Wall(3));
        tilemap.set_border(5, Right, Wall(4));

        assert_eq!(tilemap.get_border(5, Back), Wall(1));
        assert_eq!(tilemap.get_border(5, Left), Wall(2));
        assert_eq!(tilemap.get_border(5, Front), Wall(3));
        assert_eq!(tilemap.get_border(5, Right), Wall(4));
    }

    #[test]
    #[should_panic]
    fn test_set_border_outside_map() {
        let size = Size2d::new(2, 3);
        let mut tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.set_border(6, Back, Wall(1));
    }

    #[test]
    fn test_get_border_at_node() {
        let size = Size2d::new(3, 3);
        let mut tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.set_border(4, Back, Wall(1));
        tilemap.set_border(4, Left, Wall(2));

        assert_eq!(tilemap.get_border_at_node(5, Back), NoBorder);
        assert_eq!(tilemap.get_border_at_node(5, Left), NoBorder);
        assert_eq!(tilemap.get_border_at_node(5, Front), Wall(2));
        assert_eq!(tilemap.get_border_at_node(5, Right), Wall(1));

        assert_eq!(tilemap.get_border_at_node(6, Back), NoBorder);
        assert_eq!(tilemap.get_border_at_node(6, Left), Wall(1));
        assert_eq!(tilemap.get_border_at_node(6, Front), NoBorder);
        assert_eq!(tilemap.get_border_at_node(6, Right), NoBorder);

        assert_eq!(tilemap.get_border_at_node(9, Back), Wall(2));
        assert_eq!(tilemap.get_border_at_node(9, Left), NoBorder);
        assert_eq!(tilemap.get_border_at_node(9, Front), NoBorder);
        assert_eq!(tilemap.get_border_at_node(9, Right), NoBorder);

        assert_eq!(tilemap.get_border_at_node(10, Back), NoBorder);
        assert_eq!(tilemap.get_border_at_node(10, Left), NoBorder);
        assert_eq!(tilemap.get_border_at_node(10, Front), NoBorder);
        assert_eq!(tilemap.get_border_at_node(10, Right), NoBorder);
    }

    #[test]
    #[should_panic]
    fn test_get_border_at_node_outside_map() {
        let size = Size2d::new(2, 3);
        let tilemap = Tilemap2d::default(size, Empty).unwrap();

        tilemap.get_border_at_node(13, Back);
    }

    fn create_tiles() -> Vec<Tile> {
        vec![Floor(1), Empty, Solid(3), Empty, Floor(4), Empty]
    }
}
