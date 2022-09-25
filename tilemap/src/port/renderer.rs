use crate::math::color::Color;
use crate::math::size2d::Size2d;

pub trait Renderer {
    fn render_rectangle(&mut self, x: u32, y: u32, size: Size2d, color: Color);
}
