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
    tile_height: u32,
    delta: Size2d,
}

impl View for IsometricView {
    fn get_size(&self, tilemap: &Tilemap2d) -> Size2d {
        self.calculate_floor_size(tilemap.get_size()) + Size2d::new(0, self.tile_height)
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
}

impl IsometricView {
    pub fn new(tile_size: u32, tile_height: u32) -> Self {
        IsometricView {
            tile_size: Size2d::square(tile_size),
            tile_height,
            delta: Self::calculate_delta(tile_size),
        }
    }

    pub fn calculate_delta(tile_size: u32) -> Size2d {
        let delta_y = ((tile_size as f32) / (5.0_f32).sqrt()).ceil() as u32;
        Size2d::new(2 * delta_y, delta_y)
    }

    pub fn calculate_floor_size(&self, tiles: Size2d) -> Size2d {
        let left_to_center = self.delta.width() * tiles.width();
        let center_to_right = self.delta.width() * tiles.height();
        let center_to_bottom = self.delta.height() * tiles.width();
        let center_to_top = self.delta.height() * tiles.height();
        Size2d::new(
            left_to_center + center_to_right,
            center_to_bottom + center_to_top,
        )
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
        let viewer = IsometricView::new(100, 200);

        assert_eq!(viewer.get_size(&tilemap), Size2d::new(30, 75));
    }
}
