use crate::math::color::Color;
use crate::math::point2d::Point2d;
use crate::math::side::Side;
use crate::math::size2d::Size2d;
use crate::port::renderer::Renderer;
use crate::renderer::border::{calculate_horizontal_border, calculate_vertical_border};
use crate::renderer::node::{calculate_node_styles, Node};
use crate::renderer::style::aab::BoxStyle;
use crate::renderer::style::StyleMgr;
use crate::renderer::view::View;
use crate::tilemap::border::{get_vertical_borders_size, Border};
use crate::tilemap::tile::Tile;
use crate::tilemap::tilemap2d::Tilemap2d;

/// Renders a [`Tilemap2d`](crate::tilemap::tilemap2d::Tilemap2d) with an [`isometric view`](https://en.wikipedia.org/wiki/Isometric_projection).
pub struct IsometricView {
    delta: Point2d,
    tile_size: u32,
    tile_height: i32,
}

impl View for IsometricView {
    fn get_size(&self, tiles: Size2d) -> Size2d {
        self.calculate_floor_size(tiles) + Size2d::new(0, self.tile_height as u32)
    }

    fn render(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, styles: &StyleMgr) {
        let tiles = tilemap.get_size();
        let vertical_size = get_vertical_borders_size(tilemap.get_size());
        let nodes =
            calculate_node_styles(styles.get_node_styles(), styles.get_wall_styles(), tilemap);
        let mut start = self.get_start(tiles);
        let mut index = 0;

        for row in 0..tiles.height() {
            let mut point = start;

            for _x in 0..tiles.width() {
                let tile = tilemap.get_tile(index);

                match tile {
                    Tile::Empty => {}
                    Tile::Floor(id) => self.render_tile(
                        renderer,
                        point,
                        *styles.get_floor_style(id).get_floor_color(),
                    ),
                    Tile::Solid(id) => {
                        let style = styles.get_solid_style(id);
                        self.render_box(
                            renderer,
                            point,
                            self.tile_height,
                            self.delta,
                            self.delta,
                            style.get_style(),
                            (true, true),
                        )
                    }
                }

                let node_index = index + row as usize;

                match nodes[node_index] {
                    Node::NoNode => {}
                    Node::InnerNode => {}
                    Node::OuterNode(style) => {
                        let delta_half = Self::calculate_delta(style.get_half());
                        let delta_size = Self::calculate_delta(style.get_size());
                        let back = self.get_reverse_left_box(point, delta_half);
                        let back = self.get_reverse_right_box(back, delta_half);

                        self.render_box(
                            renderer,
                            back,
                            self.tile_height,
                            delta_size,
                            delta_size,
                            style.get_style(),
                            (true, true),
                        );
                    }
                }

                match tilemap.get_border(index, Side::Back) {
                    Border::NoBorder => {}
                    Border::Wall(id) => {
                        let style = styles.get_wall_style(id);

                        self.render_horizontal_border(
                            renderer,
                            &nodes,
                            point,
                            index,
                            row,
                            style.get_style(),
                            self.tile_height,
                            style.get_thickness(),
                        );
                    }
                    Border::Door(wall_id, door_id) => {
                        let wall_style = styles.get_wall_style(wall_id);
                        let door_style = styles.get_door_style(door_id);
                        let door_height = door_style.get_height() as i32;
                        let wall_height = self.tile_height - door_height;

                        self.render_horizontal_border(
                            renderer,
                            &nodes,
                            point,
                            index,
                            row,
                            door_style.get_style(),
                            door_height,
                            door_style.get_thickness(),
                        );

                        self.render_horizontal_border(
                            renderer,
                            &nodes,
                            self.get_top(point, door_height),
                            index,
                            row,
                            wall_style.get_style(),
                            wall_height,
                            wall_style.get_thickness(),
                        );
                    }
                }

                match tilemap.get_border(index, Side::Left) {
                    Border::NoBorder => {}
                    Border::Wall(id) | Border::Door(id, _) => {
                        let style = styles.get_wall_style(id);
                        let thickness = style.get_thickness();
                        let border_index = index + row as usize;
                        let (start, length) = calculate_vertical_border(
                            &nodes,
                            self.tile_size,
                            vertical_size,
                            border_index,
                        );
                        let delta_half = Self::calculate_delta(thickness / 2);
                        let delta_start = Self::calculate_delta(start as u32);
                        let left = self.get_reverse_right_box(point, delta_half);
                        let left = self.get_left_box(left, delta_start);

                        self.render_box(
                            renderer,
                            left,
                            self.tile_height,
                            Self::calculate_delta(thickness),
                            Self::calculate_delta(length),
                            style.get_style(),
                            (false, true),
                        );
                    }
                }

                // Move the point of the next tile in this row
                point = self.get_right(point);
                index += 1;
            }

            // Move the start point of the next row
            start = self.get_left(start);
        }
    }

    fn render_grid(&self, tiles: Size2d, renderer: &mut dyn Renderer, style: &StyleMgr) {
        self.render_grid_rows(tiles, renderer, style);
        self.render_grid_columns(tiles, renderer, style);
    }
}

impl IsometricView {
    pub fn new(tile_size: u32, tile_height: u32) -> Self {
        IsometricView {
            tile_size,
            delta: Self::calculate_delta(tile_size),
            tile_height: tile_height as i32,
        }
    }

    pub fn calculate_delta(size: u32) -> Point2d {
        let delta_y = Self::calculate_delta_y(size);
        Point2d::new(delta_y * 2, delta_y)
    }

    pub fn calculate_delta_y(size: u32) -> i32 {
        ((size as f32) / (5.0_f32).sqrt()).ceil() as i32
    }

    /// Calculates the size needed to render the floor of the tilemap.
    pub fn calculate_floor_size(&self, tiles: Size2d) -> Size2d {
        let dx = self.delta.x as u32;
        let dy = self.delta.y as u32;
        let left_to_center = dx * tiles.width();
        let center_to_right = dx * tiles.height();
        let center_to_bottom = dy * tiles.width();
        let center_to_top = dy * tiles.height();
        Size2d::new(
            left_to_center + center_to_right,
            center_to_bottom + center_to_top,
        )
    }

    fn render_horizontal_border(
        &self,
        renderer: &mut dyn Renderer,
        nodes: &[Node],
        point: Point2d,
        index: usize,
        row: u32,
        style: &BoxStyle,
        height: i32,
        thickness: u32,
    ) {
        let (start, length) = calculate_horizontal_border(nodes, self.tile_size, index, row);
        let delta_half = Self::calculate_delta(thickness / 2);
        let delta_start = Self::calculate_delta(start as u32);
        let back = self.get_reverse_left_box(point, delta_half);
        let back = self.get_right_box(back, delta_start);

        self.render_box(
            renderer,
            back,
            height,
            Self::calculate_delta(length),
            Self::calculate_delta(thickness),
            style,
            (true, false),
        );
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

    /// Render an axis aligned box.
    fn render_box(
        &self,
        renderer: &mut dyn Renderer,
        back: Point2d,
        height: i32,
        delta_row: Point2d,
        delta_column: Point2d,
        style: &BoxStyle,
        filter: (bool, bool),
    ) {
        self.render_ceiling(
            renderer,
            back,
            height,
            delta_row,
            delta_column,
            *style.get_top_color(),
        );

        if filter.0 {
            self.render_front(
                renderer,
                back,
                height,
                delta_row,
                delta_column,
                *style.get_front_color(),
            );
        }

        if filter.1 {
            self.render_side(
                renderer,
                back,
                height,
                delta_row,
                delta_column,
                *style.get_side_color(),
            );
        }
    }

    /// Render the top of an axis aligned box.
    fn render_ceiling(
        &self,
        renderer: &mut dyn Renderer,
        back: Point2d,
        height: i32,
        delta_row: Point2d,
        delta_column: Point2d,
        color: Color,
    ) {
        let top_back = self.get_top(back, height);
        renderer.render_transformed_rectangle(
            top_back,
            self.get_left_box(top_back, delta_column),
            self.get_front_box(top_back, delta_row, delta_column),
            self.get_right_box(top_back, delta_row),
            color,
        )
    }

    /// Render the front of an axis aligned box.
    fn render_front(
        &self,
        renderer: &mut dyn Renderer,
        back: Point2d,
        height: i32,
        delta_row: Point2d,
        delta_column: Point2d,
        color: Color,
    ) {
        let left0 = self.get_left_box(back, delta_column);
        let left1 = self.get_top(left0, height);
        let front0 = self.get_front_box(back, delta_row, delta_column);
        let front1 = self.get_top(front0, height);
        renderer.render_transformed_rectangle(left1, left0, front0, front1, color)
    }

    /// Render the side of an axis aligned box.
    fn render_side(
        &self,
        renderer: &mut dyn Renderer,
        back: Point2d,
        height: i32,
        delta_row: Point2d,
        delta_column: Point2d,
        color: Color,
    ) {
        let right0 = self.get_right_box(back, delta_row);
        let right1 = self.get_top(right0, height);
        let front0 = self.get_front_box(back, delta_row, delta_column);
        let front1 = self.get_top(front0, height);
        renderer.render_transformed_rectangle(right0, right1, front1, front0, color)
    }

    /// Render the lines of the grid between columns.
    fn render_grid_columns(&self, tiles: Size2d, renderer: &mut dyn Renderer, style: &StyleMgr) {
        let start = self.get_start(tiles);
        let mut start_column = self.get_right(start);
        let diff_column = self.get_diff_column(tiles);

        for _column in 0..(tiles.width() - 1) {
            let end_column = start_column + diff_column;

            renderer.render_line(start_column, end_column, *style.get_grid_color());

            start_column = self.get_right(start_column);
        }
    }

    /// Render the lines of the grid between rows.
    fn render_grid_rows(&self, tiles: Size2d, renderer: &mut dyn Renderer, style: &StyleMgr) {
        let start = self.get_start(tiles);
        let mut start_row = self.get_left(start);
        let diff_row = self.get_diff_row(tiles);

        for _row in 0..(tiles.height() - 1) {
            let end_row = start_row + diff_row;

            renderer.render_line(start_row, end_row, *style.get_grid_color());

            start_row = self.get_left(start_row);
        }
    }

    /// Calculate the back point of the 1.tile.
    fn get_start(&self, tiles: Size2d) -> Point2d {
        Point2d::new(self.delta.x * tiles.height() as i32, self.tile_height)
    }

    /// Calculate the front corner of the floor tile from the back corner.
    fn get_front(&self, point: Point2d) -> Point2d {
        self.get_front_box(point, self.delta, self.delta)
    }

    /// Calculate the front corner of an axis aligned box from the back corner.
    fn get_front_box(&self, point: Point2d, delta_row: Point2d, delta_column: Point2d) -> Point2d {
        Point2d::new(
            point.x + delta_row.x - delta_column.x,
            point.y + delta_row.y + delta_column.y,
        )
    }

    /// Calculate the left corner of the floor tile from the back corner.
    fn get_left(&self, point: Point2d) -> Point2d {
        self.get_left_box(point, self.delta)
    }

    /// Calculate the left corner of an axis aligned box from the back corner.
    fn get_left_box(&self, point: Point2d, delta: Point2d) -> Point2d {
        Point2d::new(point.x - delta.x, point.y + delta.y)
    }

    fn get_reverse_left_box(&self, point: Point2d, delta: Point2d) -> Point2d {
        Point2d::new(point.x + delta.x, point.y - delta.y)
    }

    /// Calculate the right corner of the floor tile from the back corner.
    fn get_right(&self, point: Point2d) -> Point2d {
        self.get_right_box(point, self.delta)
    }

    /// Calculate the right corner of an axis aligned box from the back corner.
    fn get_right_box(&self, point: Point2d, delta: Point2d) -> Point2d {
        Point2d::new(point.x + delta.x, point.y + delta.y)
    }

    fn get_reverse_right_box(&self, point: Point2d, delta: Point2d) -> Point2d {
        Point2d::new(point.x - delta.x, point.y - delta.y)
    }

    /// Calculate the equivalent point on the ceiling from any point on the floor.
    fn get_top(&self, point: Point2d, height: i32) -> Point2d {
        Point2d::new(point.x, point.y - height)
    }

    /// Calculate the difference between the start & end point of a column.
    fn get_diff_column(&self, tiles: Size2d) -> Point2d {
        self.get_left(Point2d::default()) * tiles.height()
    }

    /// Calculate the difference between the start & end point of a row.
    fn get_diff_row(&self, tiles: Size2d) -> Point2d {
        self.get_right(Point2d::default()) * tiles.width()
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

        assert_eq!(viewer.delta.x, 90);
        assert_eq!(viewer.delta.y, 45);
    }

    #[test]
    fn test_floor_size() {
        let tiles = Size2d::new(2, 3);
        let viewer = IsometricView::new(100, 2000);

        assert_eq!(viewer.calculate_floor_size(tiles), Size2d::new(450, 225));
    }

    #[test]
    fn test_get_size() {
        let viewer = IsometricView::new(100, 200);

        assert_eq!(viewer.get_size(Size2d::new(2, 3)), Size2d::new(450, 425));
    }
}
