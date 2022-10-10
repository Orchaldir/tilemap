use crate::math::color::Color;
use crate::math::point2d::Point2d;
use crate::math::size2d::Size2d;
use crate::port::renderer::Renderer;
use crate::renderer::edge::{calculate_horizontal_edge, calculate_vertical_edge};
use crate::renderer::node::{calculate_node_styles, Node};
use crate::renderer::style::StyleMgr;
use crate::renderer::view::View;
use crate::tilemap::border::{get_horizontal_borders_size, get_vertical_borders_size, Border};
use crate::tilemap::node::get_nodes_size;
use crate::tilemap::tile::Tile;
use crate::tilemap::tilemap2d::Tilemap2d;

/// Renders a [`Tilemap2d`](crate::tilemap::tilemap2d::Tilemap2d) with a top-down view.
pub struct TopDownView {
    tile_size: Size2d,
}

impl View for TopDownView {
    fn get_size(&self, tiles: Size2d) -> Size2d {
        tiles * self.tile_size
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

        for row in 0..(tiles.height() - 1) {
            let y = ((row + 1) * self.tile_size.height()) as i32;
            renderer.render_line(
                Point2d::new(0, y),
                Point2d::new(size.width() as i32, y),
                *styles.get_grid_color(),
            );
        }

        for column in 0..(tiles.width() - 1) {
            let x = ((column + 1) * self.tile_size.width()) as i32;
            renderer.render_line(
                Point2d::new(x, 0),
                Point2d::new(x, size.height() as i32),
                *styles.get_grid_color(),
            );
        }
    }
}

impl TopDownView {
    pub fn new(tile_size: Size2d) -> Self {
        TopDownView { tile_size }
    }

    fn render_tiles(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, styles: &StyleMgr) {
        let tiles = tilemap.get_size();
        let mut y = 0;
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
                        let color = styles.get_solid_style(id).get_aab_style().get_top_color();
                        self.render_tile(renderer, x, y, *color)
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

        let mut y = 0i32;
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

                        renderer.render_rectangle(
                            x + start,
                            y - thickness as i32 / 2,
                            Size2d::new(length, thickness),
                            *style.get_aab_style().get_top_color(),
                        )
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
            let mut x = 0i32;

            for _x in 0..size.width() {
                match &borders[index] {
                    Border::NoBorder => {}
                    Border::Wall(id) => {
                        let style = styles.get_wall_style(*id);
                        let thickness = style.get_thickness();
                        let (start, length) =
                            calculate_vertical_edge(nodes, self.tile_size.width(), size, index);

                        renderer.render_rectangle(
                            x - thickness as i32 / 2,
                            y + start,
                            Size2d::new(thickness, length),
                            *style.get_aab_style().get_top_color(),
                        )
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
            let mut x = 0i32;

            for _x in 0..size.width() {
                match nodes[index] {
                    Node::NoNode => {}
                    Node::OuterNode(style) => {
                        let half = style.get_half() as i32;

                        renderer.render_rectangle(
                            x - half,
                            y - half,
                            Size2d::square(style.get_size()),
                            *style.get_style().get_top_color(),
                        )
                    }
                }

                x += self.tile_size.width() as i32;
                index += 1;
            }

            y += self.tile_size.height() as i32;
        }
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
        let viewer = TopDownView::new(Size2d::new(15, 25));

        assert_eq!(viewer.get_size(Size2d::new(2, 3)), Size2d::new(30, 75));
    }
}
