use crate::math::color::Color;
use crate::math::size2d::Size2d;
use crate::port::renderer::Renderer;
use crate::renderer::style::Style;
use crate::renderer::view::View;
use crate::tilemap::tile::Tile;
use crate::tilemap::tilemap2d::Tilemap2d;

/// Renders a [`Tilemap2d`](crate::tilemap::tilemap2d::Tilemap2d) with a 3/4 view.
pub struct ThreeFourView {
    tile_size: Size2d,
    tile_height: u32,
}

impl View for ThreeFourView {
    fn get_size(&self, tiles: Size2d) -> Size2d {
        tiles * self.tile_size + Size2d::new(0, self.tile_height)
    }

    fn render(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, style: &Style) {
        let tiles = tilemap.get_size();
        let front = Size2d::new(self.tile_size.width(), self.tile_height);
        let mut y = self.tile_height;
        let mut index = 0;

        for _y in 0..tiles.height() {
            let mut x = 0;

            for _x in 0..tiles.width() {
                let tile = tilemap.get_tile(index);

                match tile {
                    Tile::Empty => {}
                    Tile::Floor(_id) => self.render_tile(renderer, x, y, *style.get_floor_color()),
                    Tile::Solid(_id) => {
                        let top_y = y - self.tile_height;
                        let front_y = top_y + self.tile_size.height();
                        renderer.render_rectangle(x, front_y, front, *style.get_front_color());
                        self.render_tile(renderer, x, top_y, *style.get_top_color());
                    }
                }

                x += self.tile_size.width();
                index += 1;
            }

            y += self.tile_size.height();
        }
    }

    fn render_grid(&self, tiles: Size2d, renderer: &mut dyn Renderer, style: &Style) {}
}

impl ThreeFourView {
    pub fn new(tile_size: Size2d, tile_height: u32) -> Self {
        ThreeFourView {
            tile_size,
            tile_height,
        }
    }

    fn render_tile(&self, renderer: &mut dyn Renderer, x: u32, y: u32, color: Color) {
        renderer.render_rectangle(x, y, self.tile_size, color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_size() {
        let viewer = ThreeFourView::new(Size2d::new(15, 25), 35);

        assert_eq!(viewer.get_size(Size2d::new(2, 3)), Size2d::new(30, 110));
    }
}
