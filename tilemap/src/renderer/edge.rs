use crate::math::size2d::Size2d;
use crate::renderer::node::Node;
use crate::tilemap::node::{
    get_end_of_horizontal_border, get_end_of_vertical_border, get_start_of_horizontal_border,
    get_start_of_vertical_border,
};

pub fn calculate_horizontal_edge(
    nodes: &[Node],
    tile_size: u32,
    border_index: usize,
    y: u32,
) -> (i32, u32) {
    let start_index = get_start_of_horizontal_border(border_index, y);
    let end_index = get_end_of_horizontal_border(border_index, y);
    calculate_edge(nodes, tile_size, start_index, end_index)
}

pub fn calculate_vertical_edge(
    nodes: &[Node],
    tile_size: u32,
    size: Size2d,
    border_index: usize,
) -> (i32, u32) {
    let start_index = get_start_of_vertical_border(border_index);
    let end_index = get_end_of_vertical_border(size, border_index);
    calculate_edge(nodes, tile_size, start_index, end_index)
}

fn calculate_edge(
    nodes: &[Node],
    tile_size: u32,
    start_index: usize,
    end_index: usize,
) -> (i32, u32) {
    let start_half = nodes[start_index].calculate_half();
    let end_half = nodes[end_index].calculate_half();
    (start_half as i32, tile_size - (start_half + end_half))
}
