use crate::math::color::Color;
use crate::math::point2d::Point2d;
use crate::math::size2d::Size2d;
use crate::port::renderer::Renderer;
use crate::renderer::style::Style;
use crate::renderer::view::View;
use crate::tilemap::tile::Tile;
use crate::tilemap::tilemap2d::Tilemap2d;

/// Renders a [`Tilemap2d`](crate::tilemap::tilemap2d::Tilemap2d) with an isometric view.
pub struct IsometricView {
    delta: Size2d,
    tile_height: u32,
}

impl View for IsometricView {
    fn get_size(&self, tilemap: &Tilemap2d) -> Size2d {
        self.calculate_floor_size(tilemap.get_size()) + Size2d::new(0, self.tile_height)
    }

    fn render(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, style: &Style) {
        let tiles = tilemap.get_size();
        let mut start = Point2d::new(
            (self.delta.width() * tiles.height()) as i32,
            self.tile_height as i32,
        );
        let mut index = 0;

        for _y in 0..tiles.height() {
            let mut point = start;

            for _x in 0..tiles.width() {
                let tile = tilemap.get_tile(index);

                match tile {
                    Tile::Empty => {}
                    Tile::Floor(_id) => self.render_tile(renderer, point, *style.get_floor_color()),
                    Tile::Solid(_id) => self.render_tile(renderer, point, *style.get_top_color()),
                }

                // Move the point of the next tile in this row row
                point.x += self.delta.width() as i32;
                point.y += self.delta.height() as i32;
                index += 1;
            }

            // Move the start point of the next row
            start.x -= self.delta.width() as i32;
            start.y += self.delta.height() as i32;
        }
    }
}

impl IsometricView {
    pub fn new(tile_size: u32, tile_height: u32) -> Self {
        IsometricView {
            delta: Self::calculate_delta(tile_size),
            tile_height,
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

    fn render_tile(&self, renderer: &mut dyn Renderer, top: Point2d, color: Color) {
        let left = Point2d::new(
            top.x - self.delta.width() as i32,
            top.y + self.delta.height() as i32,
        );
        let right = Point2d::new(
            top.x + self.delta.width() as i32,
            top.y + self.delta.height() as i32,
        );
        let bottom = Point2d::new(top.x, top.y + self.delta.height() as i32 * 2);
        renderer.render_transformed_rectangle(top, left, bottom, right, color)
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
