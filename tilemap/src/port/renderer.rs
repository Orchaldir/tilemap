use crate::math::color::Color;
use crate::math::point2d::Point2d;
use crate::math::size2d::Size2d;

pub trait Renderer {
    /// Renders an axis aligned rectangle.
    fn render_rectangle(&mut self, x: u32, y: u32, size: Size2d, color: Color);

    /// Renders a transformed rectangle. The points are ordered counter-clockwise.
    fn render_transformed_rectangle(
        &mut self,
        p0: Point2d,
        p1: Point2d,
        p2: Point2d,
        p3: Point2d,
        color: Color,
    );

    /// Renders a line between 2 points.
    fn render_line(&mut self, p0: Point2d, p1: Point2d, color: Color);
}
