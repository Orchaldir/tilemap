use crate::math::color::Color;
use crate::math::point2d::Point2d;
use crate::math::size2d::Size2d;
use crate::port::renderer::Renderer;
use crate::renderer::edge::{calculate_horizontal_edge, calculate_vertical_edge};
use crate::renderer::node::{calculate_node_styles, Node};
use crate::renderer::style::aab::BoxStyle;
use crate::renderer::style::StyleMgr;
use crate::renderer::view::View;
use crate::tilemap::border::{get_horizontal_borders_size, get_vertical_borders_size, Border};
use crate::tilemap::node::get_nodes_size;
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

    fn render(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, styles: &StyleMgr) {
        self.render_tiles(tilemap, renderer, styles);

        let nodes =
            calculate_node_styles(styles.get_node_styles(), styles.get_wall_styles(), tilemap);

        self.render_horizontal_borders(tilemap, &nodes, renderer, styles);
        self.render_vertical_borders(tilemap, &nodes, renderer, styles);
        self.render_nodes(tilemap, &nodes, renderer);
    }

    fn render_grid(&self, tiles: Size2d, renderer: &mut dyn Renderer, styles: &StyleMgr) {
        let size = self.get_size(tiles);
        let mut y = self.tile_height + self.tile_size.height();

        for _row in 0..(tiles.height() - 1) {
            renderer.render_line(
                Point2d::new(0, y as i32),
                Point2d::new(size.width() as i32, y as i32),
                *styles.get_grid_color(),
            );

            y += self.tile_size.height();
        }

        y = self.tile_height;

        for column in 0..(tiles.width() - 1) {
            let x = ((column + 1) * self.tile_size.width()) as i32;
            renderer.render_line(
                Point2d::new(x, y as i32),
                Point2d::new(x, size.height() as i32),
                *styles.get_grid_color(),
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

    fn render_tiles(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, styles: &StyleMgr) {
        let tiles = tilemap.get_size();
        let mut y = self.tile_height as i32;
        let mut index = 0;

        for _y in 0..tiles.height() {
            let mut x = 0;

            for _x in 0..tiles.width() {
                let tile = tilemap.get_tile(index);

                match tile {
                    Tile::Empty => {}
                    Tile::Floor(id) => self.render_tile(
                        renderer,
                        x,
                        y,
                        *styles.get_floor_style(id).get_floor_color(),
                    ),
                    Tile::Solid(id) => {
                        self.render_aabb(
                            renderer,
                            x,
                            y - self.tile_height as i32,
                            self.tile_size.width(),
                            self.tile_size.height(),
                            styles.get_solid_style(id).get_aab_style(),
                        );
                    }
                }

                x += self.tile_size.width() as i32;
                index += 1;
            }

            y += self.tile_size.height() as i32;
        }
    }

    fn render_horizontal_borders(
        &self,
        tilemap: &Tilemap2d,
        nodes: &[Node],
        renderer: &mut dyn Renderer,
        styles: &StyleMgr,
    ) {
        let size = get_horizontal_borders_size(tilemap.get_size());
        let borders = tilemap.get_horizontal_borders();

        let mut y = 0;
        let mut index = 0;

        for row in 0..size.height() {
            let mut x = 0;

            for _x in 0..size.width() {
                match &borders[index] {
                    Border::NoBorder => {}
                    Border::Wall(id) => {
                        let style = styles.get_wall_style(*id);
                        let thickness = style.get_thickness();
                        let (start, length) =
                            calculate_horizontal_edge(nodes, self.tile_size.width(), index, row);

                        self.render_aabb(
                            renderer,
                            x + start,
                            y - thickness as i32 / 2,
                            length,
                            thickness,
                            style.get_aab_style(),
                        );
                    }
                }

                x += self.tile_size.width() as i32;
                index += 1;
            }

            y += self.tile_size.height() as i32;
        }
    }

    fn render_vertical_borders(
        &self,
        tilemap: &Tilemap2d,
        nodes: &[Node],
        renderer: &mut dyn Renderer,
        styles: &StyleMgr,
    ) {
        let size = get_vertical_borders_size(tilemap.get_size());
        let borders = tilemap.get_vertical_borders();

        let mut y = 0;
        let mut index = 0;

        for _y in 0..size.height() {
            let mut x = 0;

            for _x in 0..size.width() {
                match &borders[index] {
                    Border::NoBorder => {}
                    Border::Wall(id) => {
                        let style = styles.get_wall_style(*id);
                        let thickness = style.get_thickness();
                        let (start, length) =
                            calculate_vertical_edge(nodes, self.tile_size.width(), size, index);

                        self.render_aabb(
                            renderer,
                            x - thickness as i32 / 2,
                            y + start,
                            thickness,
                            length,
                            style.get_aab_style(),
                        );
                    }
                }

                x += self.tile_size.width() as i32;
                index += 1;
            }

            y += self.tile_size.height() as i32;
        }
    }

    fn render_nodes(&self, tilemap: &Tilemap2d, nodes: &[Node], renderer: &mut dyn Renderer) {
        let size = get_nodes_size(tilemap.get_size());

        let mut y = 0;
        let mut index = 0;

        for _y in 0..size.height() {
            let mut x = 0;

            for _x in 0..size.width() {
                match nodes[index] {
                    Node::NoNode => {}
                    Node::InnerNode => {}
                    Node::OuterNode(style) => {
                        let half = style.get_half() as i32;

                        self.render_aabb(
                            renderer,
                            x - half,
                            y - half,
                            style.get_size(),
                            style.get_size(),
                            style.get_style(),
                        );
                    }
                }

                x += self.tile_size.width() as i32;
                index += 1;
            }

            y += self.tile_size.height() as i32;
        }
    }

    fn render_aabb(
        &self,
        renderer: &mut dyn Renderer,
        x: i32,
        y: i32,
        size_x: u32,
        size_y: u32,
        style: &BoxStyle,
    ) {
        // render top

        renderer.render_rectangle(x, y, Size2d::new(size_x, size_y), *style.get_top_color());

        // render front

        renderer.render_rectangle(
            x,
            y + size_y as i32,
            Size2d::new(size_x, self.tile_height),
            *style.get_front_color(),
        );
    }

    fn render_tile(&self, renderer: &mut dyn Renderer, x: i32, y: i32, color: Color) {
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
