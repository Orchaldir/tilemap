use crate::math::size2d::Size2d;
use crate::tilemap::tilemap2d::Tilemap2d;

pub mod three_four;
pub mod top_down;

pub trait View {
    /// Returns the required size to fully render the tilemap.
    fn get_size(&self, tilemap: &Tilemap2d) -> Size2d;
}
