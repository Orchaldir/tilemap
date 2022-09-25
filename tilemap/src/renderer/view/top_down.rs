use crate::math::color::{Color, BLACK, CYAN};
use crate::math::size2d::Size2d;
use crate::port::renderer::Renderer;
use crate::tilemap::tile::Tile;
use crate::tilemap::tilemap2d::Tilemap2d;

/// Renders a [`Tilemap2d`](crate::tilemap::tilemap2d::Tilemap2d) with a top-down view.
pub struct TopDownView {
    tile_size: Size2d,
}

impl TopDownView {
    pub fn new(tile_size: Size2d) -> Self {
        TopDownView { tile_size }
    }

    pub fn get_tile_size(&self) -> Size2d {
        self.tile_size
    }

    /// Renders a [`Tilemap2d`](crate::tilemap::tilemap2d::Tilemap2d) with a specific [`renderer`](crate::port::renderer::Renderer).
    pub fn render(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer) {
        let tiles = tilemap.get_size();
        let mut y = 0;
        let mut index = 0;

        for _y in 0..tiles.height() {
            let mut x = 0;

            for _x in 0..tiles.width() {
                let tile = tilemap.get_tile(index);

                match tile {
                    Tile::Empty => {}
                    Tile::Floor(_id) => self.render_tile(renderer, x, y, CYAN),
                    Tile::Solid(_id) => self.render_tile(renderer, x, y, BLACK),
                }

                x += self.tile_size.width();
                index += 1;
            }

            y += self.tile_size.height();
        }
    }

    fn render_tile(&self, renderer: &mut dyn Renderer, x: u32, y: u32, color: Color) {
        renderer.render_rectangle(x, y, self.tile_size, color)
    }
}
