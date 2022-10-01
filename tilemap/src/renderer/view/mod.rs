use crate::math::size2d::Size2d;
use crate::port::renderer::Renderer;
use crate::renderer::style::Style;
use crate::tilemap::tilemap2d::Tilemap2d;

pub mod isometric;
pub mod three_four;
pub mod top_down;

pub trait View {
    /// Returns the required size to fully render the tilemap.
    fn get_size(&self, tiles: Size2d) -> Size2d;

    /// Renders a [`tilemap`](crate::tilemap::tilemap2d::Tilemap2d) with a specific [`renderer`](crate::port::renderer::Renderer)
    /// & [`style`](crate::renderer::style::Style).
    fn render(&self, tilemap: &Tilemap2d, renderer: &mut dyn Renderer, style: &Style);

    /// Renders the grid for the tiles.
    fn render_grid(&self, tiles: Size2d, renderer: &mut dyn Renderer, style: &Style);
}
