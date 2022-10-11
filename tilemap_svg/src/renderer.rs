use anyhow::Result;
use std::fs::File;
use std::io::Write;
use tilemap::math::color::Color;
use tilemap::math::point2d::Point2d;
use tilemap::math::size2d::Size2d;
use tilemap::port::renderer::Renderer;

/// A valid [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics).
#[derive(Debug, PartialEq, Eq)]
pub struct Svg {
    lines: Vec<String>,
}

impl Svg {
    pub fn export(&self) -> String {
        self.lines.join("\n")
    }

    /// Save the svg to a file.
    pub fn save(&self, path: &str) -> Result<()> {
        let mut output = File::create(path)?;

        for line in &self.lines {
            writeln!(&mut output, "{}", line)?;
        }

        Ok(())
    }
}

/// Builds a valid [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics).
#[derive(Debug, PartialEq, Eq)]
pub struct SvgBuilder {
    lines: Vec<String>,
}

impl SvgBuilder {
    pub fn new(size: Size2d) -> Self {
        let mut lines = Vec::new();

        lines.push(format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            size.width(),
            size.height()
        ));

        Self { lines }
    }

    pub fn finish(mut self) -> Svg {
        self.lines.push("</svg>".to_string());

        Svg { lines: self.lines }
    }
}

impl Renderer for SvgBuilder {
    fn render_rectangle(&mut self, x: i32, y: i32, size: Size2d, color: Color) {
        self.lines.push(format!(
            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>",
            x,
            y,
            size.width(),
            size.height(),
            color.to_hex(),
        ));
    }

    fn render_transformed_rectangle(
        &mut self,
        p0: Point2d,
        p1: Point2d,
        p2: Point2d,
        p3: Point2d,
        color: Color,
    ) {
        self.lines.push(format!(
            "  <polygon points=\"{},{} {},{} {},{} {},{}\" fill=\"{}\"/>",
            p0.x,
            p0.y,
            p1.x,
            p1.y,
            p2.x,
            p2.y,
            p3.x,
            p3.y,
            color.to_hex(),
        ));
    }

    fn render_line(&mut self, p0: Point2d, p1: Point2d, color: Color) {
        self.lines.push(format!(
            "  <path d=\"M{} {} {} {}\" stroke=\"{}\" stroke-dasharray=\"1,4\"/>",
            p0.x,
            p0.y,
            p1.x,
            p1.y,
            color.to_hex(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tilemap::math::color::{ORANGE, PINK};

    #[test]
    fn test_empty_svg() {
        let builder = SvgBuilder::new(Size2d::new(100, 150));
        let svg = builder.finish();

        let result = "<svg viewBox=\"0 0 100 150\" xmlns=\"http://www.w3.org/2000/svg\">\n</svg>";

        assert_eq!(&svg.export(), result);
    }

    #[test]
    fn test_rectangles() {
        let mut builder = SvgBuilder::new(Size2d::new(100, 150));
        builder.render_rectangle(10, 20, Size2d::new(30, 40), ORANGE);
        builder.render_rectangle(50, 70, Size2d::new(35, 45), PINK);
        let svg = builder.finish();

        let result = "<svg viewBox=\"0 0 100 150\" xmlns=\"http://www.w3.org/2000/svg\">
  <rect x=\"10\" y=\"20\" width=\"30\" height=\"40\" fill=\"#FFA500\"/>
  <rect x=\"50\" y=\"70\" width=\"35\" height=\"45\" fill=\"#FF0080\"/>
</svg>";

        assert_eq!(&svg.export(), result);
    }
}
