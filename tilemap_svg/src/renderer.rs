use tilemap::math::size2d::Size2d;

/// A valid [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics).
#[derive(Debug, PartialEq, Eq)]
pub struct Svg {
    lines: Vec<String>,
}

impl Svg {
    pub fn export(&self) -> String {
        self.lines.join("\n")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_svg() {
        let builder = SvgBuilder::new(Size2d::new(100, 150));
        let svg = builder.finish();

        let result = "<svg viewBox=\"0 0 100 150\" xmlns=\"http://www.w3.org/2000/svg\">\n</svg>";

        assert_eq!(&svg.export(), result);
    }
}
