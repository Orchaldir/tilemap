use crate::math::size2d::Size2d;

/// Returns the [`size`](Size2d) of the nodes based on the size of the [`tilemap`](crate::tilemap::tilemap2d::Tilemap2d).
pub fn get_nodes_size(size: Size2d) -> Size2d {
    Size2d::new(size.width() + 1, size.height() + 1)
}
