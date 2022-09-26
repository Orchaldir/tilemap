use crate::math::color::Color;
use crate::math::size2d::Size2d;
use crate::port::renderer::Renderer;
use crate::renderer::style::Style;
use crate::tilemap::tile::Tile;
use crate::tilemap::tilemap2d::Tilemap2d;

/// Renders a [`Tilemap2d`](crate::tilemap::tilemap2d::Tilemap2d) with a 3/4 view.
pub struct ThreeFourView {
    tile_size: Size2d,
    tile_height: u32,
}

impl ThreeFourView {
    pub fn new(tile_size: Size2d, tile_height: u32) -> Self {
        ThreeFourView {
            tile_size,
            tile_height,
        }
    }

    /// Renders a [`Tilemap2d`](crate::tilemap::tilemap2d::Tilemap2d) with a specific [`renderer`](crate::port::renderer::Renderer).
    pub fn render(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, style: &Style) {
        let tiles = tilemap.get_size();
        let mut y = 0;
        let mut index = 0;

        for _y in 0..tiles.height() {
            let mut x = 0;

            for _x in 0..tiles.width() {
                let tile = tilemap.get_tile(index);

                match tile {
                    Tile::Empty => {}
                    Tile::Floor(_id) => self.render_tile(renderer, x, y, *style.get_floor_color()),
                    Tile::Solid(_id) => self.render_tile(renderer, x, y, *style.get_top_color()),
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
