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
    delta_x: i32,
    delta_y: i32,
    tile_height: i32,
}

impl View for IsometricView {
    fn get_size(&self, tilemap: &Tilemap2d) -> Size2d {
        self.calculate_floor_size(tilemap.get_size()) + Size2d::new(0, self.tile_height as u32)
    }

    fn render(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, style: &Style) {
        let tiles = tilemap.get_size();
        let mut start = Point2d::new(self.delta_x * tiles.height() as i32, self.tile_height);
        let mut index = 0;

        for _y in 0..tiles.height() {
            let mut point = start;

            for _x in 0..tiles.width() {
                let tile = tilemap.get_tile(index);

                match tile {
                    Tile::Empty => {}
                    Tile::Floor(_id) => self.render_tile(renderer, point, *style.get_floor_color()),
                    Tile::Solid(_id) => {
                        self.render_ceiling(renderer, point, *style.get_top_color());
                        self.render_front(renderer, point, *style.get_front_color());
                        self.render_side(renderer, point, *style.get_side_color());
                    }
                }

                // Move the point of the next tile in this row row
                point.x += self.delta_x as i32;
                point.y += self.delta_y as i32;
                index += 1;
            }

            // Move the start point of the next row
            start.x -= self.delta_x as i32;
            start.y += self.delta_y as i32;
        }
    }
}

impl IsometricView {
    pub fn new(tile_size: u32, tile_height: u32) -> Self {
        let delta_y = Self::calculate_delta_y(tile_size);
        IsometricView {
            delta_x: delta_y * 2,
            delta_y,
            tile_height: tile_height as i32,
        }
    }

    pub fn calculate_delta_y(tile_size: u32) -> i32 {
        ((tile_size as f32) / (5.0_f32).sqrt()).ceil() as i32
    }

    pub fn calculate_floor_size(&self, tiles: Size2d) -> Size2d {
        let dx = self.delta_x as u32;
        let dy = self.delta_y as u32;
        let left_to_center = dx * tiles.width();
        let center_to_right = dx * tiles.height();
        let center_to_bottom = dy * tiles.width();
        let center_to_top = dy * tiles.height();
        Size2d::new(
            left_to_center + center_to_right,
            center_to_bottom + center_to_top,
        )
    }

    fn render_tile(&self, renderer: &mut dyn Renderer, top: Point2d, color: Color) {
        renderer.render_transformed_rectangle(
            top,
            self.get_left(top),
            self.get_bottom(top),
            self.get_right(top),
            color,
        )
    }

    fn render_ceiling(&self, renderer: &mut dyn Renderer, point: Point2d, color: Color) {
        let ceiling = Point2d::new(point.x, point.y - self.tile_height);
        self.render_tile(renderer, ceiling, color)
    }

    fn render_front(&self, renderer: &mut dyn Renderer, point: Point2d, color: Color) {
        let left = Point2d::new(point.x - self.delta_x, point.y + self.delta_y);
        let bottom = Point2d::new(point.x, point.y + self.delta_y * 2);
        let right = Point2d::new(bottom.x, bottom.y - self.tile_height);
        let top = Point2d::new(left.x, left.y - self.tile_height);
        renderer.render_transformed_rectangle(top, left, bottom, right, color)
    }

    fn render_side(&self, renderer: &mut dyn Renderer, point: Point2d, color: Color) {
        let right = Point2d::new(point.x + self.delta_x, point.y + self.delta_y);
        let top = Point2d::new(right.x, right.y - self.tile_height);
        let bottom = Point2d::new(point.x, point.y + self.delta_y * 2);
        let left = Point2d::new(bottom.x, bottom.y - self.tile_height);
        renderer.render_transformed_rectangle(right, top, left, bottom, color)
    }

    fn get_bottom(&self, point: Point2d) -> Point2d {
        Point2d::new(point.x, point.y + self.delta_y * 2)
    }

    fn get_left(&self, point: Point2d) -> Point2d {
        Point2d::new(point.x - self.delta_x, point.y + self.delta_y)
    }

    fn get_right(&self, point: Point2d) -> Point2d {
        Point2d::new(point.x + self.delta_x, point.y + self.delta_y)
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
