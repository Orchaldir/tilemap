use crate::math::color::Color;
use crate::math::size2d::Size2d;
use crate::port::renderer::Renderer;
use crate::renderer::style::Style;
use crate::renderer::view::View;
use crate::tilemap::tile::Tile;
use crate::tilemap::tilemap2d::Tilemap2d;

/// Renders a [`Tilemap2d`](crate::tilemap::tilemap2d::Tilemap2d) with an isometric view.
pub struct IsometricView {
    tile_size: Size2d,
}

impl View for IsometricView {
    fn get_size(&self, tilemap: &Tilemap2d) -> Size2d {
        tilemap.get_size() * self.tile_size
    }

    fn render(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, style: &Style) {
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
}

impl IsometricView {
    pub fn new(tile_size: Size2d) -> Self {
        IsometricView { tile_size }
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
        let tilemap = Tilemap2d::default(Size2d::new(2, 3), Tile::Empty).unwrap();
        let viewer = IsometricView::new(Size2d::new(15, 25));

        assert_eq!(viewer.get_size(&tilemap), Size2d::new(30, 75));
    }
}
