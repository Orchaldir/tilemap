use crate::math::size2d::Size2d;

/// Returns the [`size`](Size2d) of the [`nodes`](crate::renderer::node::Node) based on the size of the [`tilemap`](crate::tilemap::tilemap2d::Tilemap2d).
pub fn get_nodes_size(size: Size2d) -> Size2d {
    Size2d::new(size.width() + 1, size.height() + 1)
}

/// Returns the index of the [`node`](crate::renderer::node::Node) at the start of the horizontal [`border`](crate::tilemap::border::Border).
pub fn get_start_of_horizontal_border(border_index: usize, y: u32) -> usize {
    border_index + y as usize
}

/// Returns the index of the [`node`](crate::renderer::node::Node) at the end of the horizontal [`border`](crate::tilemap::border::Border).
pub fn get_end_of_horizontal_border(border_index: usize, y: u32) -> usize {
    border_index + y as usize + 1
}

/// Returns the index of the [`node`](crate::renderer::node::Node) at the start of the vertical [`border`](crate::tilemap::border::Border).
pub fn get_start_of_vertical_border(border_index: usize) -> usize {
    border_index
}

/// Returns the index of the [`node`](crate::renderer::node::Node) at the end of the vertical [`border`](crate::tilemap::border::Border).
pub fn get_end_of_vertical_border(size: Size2d, border_index: usize) -> usize {
    border_index + size.width() as usize
}
