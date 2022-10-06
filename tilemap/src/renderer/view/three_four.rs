use crate::math::color::Color;
use crate::math::point2d::Point2d;
use crate::math::size2d::Size2d;
use crate::port::renderer::Renderer;
use crate::renderer::style::Style;
use crate::renderer::view::View;
use crate::tilemap::border::{get_horizontal_borders_size, Border};
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
        self.render_tiles(tilemap, renderer, style);
        self.render_horizontal_borders(tilemap, renderer, style);
    }

    fn render_grid(&self, tiles: Size2d, renderer: &mut dyn Renderer, style: &Style) {
        let size = self.get_size(tiles);
        let mut y = self.tile_height + self.tile_size.height();

        for _row in 0..(tiles.height() - 1) {
            renderer.render_line(
                Point2d::new(0, y as i32),
                Point2d::new(size.width() as i32, y as i32),
                *style.get_grid_color(),
            );

            y += self.tile_size.height();
        }

        y = self.tile_height;

        for column in 0..(tiles.width() - 1) {
            let x = ((column + 1) * self.tile_size.width()) as i32;
            renderer.render_line(
                Point2d::new(x, y as i32),
                Point2d::new(x, size.height() as i32),
                *style.get_grid_color(),
            );
        }
    }
}

impl ThreeFourView {
    pub fn new(tile_size: Size2d, tile_height: u32) -> Self {
        ThreeFourView {
            tile_size,
            tile_height,
        }
    }

    fn render_tiles(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, style: &Style) {
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

    fn render_horizontal_borders(
        &self,
        tilemap: &Tilemap2d,
        renderer: &mut dyn Renderer,
        style: &Style,
    ) {
        let size = get_horizontal_borders_size(tilemap.get_size());
        let borders = tilemap.get_horizontal_borders();
        let front = Size2d::new(self.tile_size.width(), self.tile_height);

        let mut y = 0i32;
        let mut index = 0;

        for _y in 0..size.height() {
            let mut x = 0;

            for _x in 0..size.width() {
                match &borders[index] {
                    Border::Empty => {}
                    Border::Wall(_) => {
                        let thickness = style.get_wall_thickness();

                        // render top

                        renderer.render_rectangle(
                            x,
                            (y - thickness as i32 / 2) as u32,
                            Size2d::new(self.tile_size.width(), thickness),
                            *style.get_top_color(),
                        );

                        // render front

                        renderer.render_rectangle(
                            x,
                            (y + thickness as i32 / 2) as u32,
                            front,
                            *style.get_front_color(),
                        );
                    }
                }

                x += self.tile_size.width();
                index += 1;
            }

            y += self.tile_size.height() as i32;
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
