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

                // Move the point of the next tile in this row
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

    /// Calculates the size needed to render the floor of the tilemap.
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

    /// Render a tile relative to its back point.
    fn render_tile(&self, renderer: &mut dyn Renderer, back: Point2d, color: Color) {
        renderer.render_transformed_rectangle(
            back,
            self.get_left(back),
            self.get_front(back),
            self.get_right(back),
            color,
        )
    }

    /// Render the ceiling tile relative to the back point of the floor tile.
    fn render_ceiling(&self, renderer: &mut dyn Renderer, back: Point2d, color: Color) {
        self.render_tile(renderer, self.get_top(back), color)
    }

    /// Render the front of a solid tile relative to the back point of the floor tile.
    fn render_front(&self, renderer: &mut dyn Renderer, back: Point2d, color: Color) {
        let left0 = self.get_left(back);
        let left1 = self.get_top(left0);
        let front0 = self.get_front(back);
        let front1 = self.get_top(front0);
        renderer.render_transformed_rectangle(left1, left0, front0, front1, color)
    }

    /// Render the side of a solid tile relative to the back point of the floor tile.
    fn render_side(&self, renderer: &mut dyn Renderer, back: Point2d, color: Color) {
        let right0 = self.get_right(back);
        let right1 = self.get_top(right0);
        let front0 = self.get_front(back);
        let front1 = self.get_top(front0);
        renderer.render_transformed_rectangle(right0, right1, front1, front0, color)
    }

    /// Calculate the front corner of the floor tile from the back corner.
    fn get_front(&self, point: Point2d) -> Point2d {
        Point2d::new(point.x, point.y + self.delta_y * 2)
    }

    /// Calculate the left corner of the floor tile from the back corner.
    fn get_left(&self, point: Point2d) -> Point2d {
        Point2d::new(point.x - self.delta_x, point.y + self.delta_y)
    }

    /// Calculate the right corner of the floor tile from the back corner.
    fn get_right(&self, point: Point2d) -> Point2d {
        Point2d::new(point.x + self.delta_x, point.y + self.delta_y)
    }

    /// Calculate the equivalent point on the ceiling from any point on the floor.
    fn get_top(&self, point: Point2d) -> Point2d {
        Point2d::new(point.x, point.y - self.tile_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_delta_y() {
        assert_eq!(IsometricView::calculate_delta_y(100), 45);
    }

    #[test]
    fn test_delta() {
        let viewer = IsometricView::new(100, 2000);

        assert_eq!(viewer.delta_x, 90);
        assert_eq!(viewer.delta_y, 45);
    }

    #[test]
    fn test_floor_size() {
        let tiles = Size2d::new(2, 3);
        let viewer = IsometricView::new(100, 2000);

        assert_eq!(viewer.calculate_floor_size(tiles), Size2d::new(450, 225));
    }

    #[test]
    fn test_get_size() {
        let tilemap = Tilemap2d::default(Size2d::new(2, 3), Tile::Empty).unwrap();
        let viewer = IsometricView::new(100, 200);

        assert_eq!(viewer.get_size(&tilemap), Size2d::new(450, 425));
    }
}
