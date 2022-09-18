use std::ops::Mul;

#[svgbobdoc::transform]
/// Defines a grid of tiles in 2 dimensions.
///
/// # Diagram
///
/// ```svgbob
///       0   1
///   +----------> x-axis
///   |
///   | +---+---+
/// 0 | | 0 | 1 |
///   | +---+---+
/// 1 | | 2 | 3 |
///   | +---+---+
/// 2 | | 4 | 5 |
///   | +---+---+
///   v
/// y-axis
/// ```
///
/// An example size with width 2 & height 3.
/// The numbers are indices of each tile.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Size2d {
    width: u32,
    height: u32,
}

impl Size2d {
    /// Returns a new size.
    pub const fn new(width: u32, height: u32) -> Size2d {
        Size2d { width, height }
    }

    /// Returns a size with equal width & height.
    pub const fn square(size: u32) -> Size2d {
        Size2d::new(size, size)
    }

    /// Returns a new size with switched width & height.
    ///
    /// ```
    ///# use tilemap::math::size2d::Size2d;
    /// let size = Size2d::new(10, 30);
    /// assert_eq!(size.flip(), Size2d::new(30, 10));
    /// ```
    pub fn flip(&self) -> Size2d {
        Size2d::new(self.height, self.width)
    }

    /// Returns the number of tiles covered by this size.
    ///
    /// ```
    ///# use tilemap::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.count(), 6);
    /// ```
    pub fn count(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// Returns the size along the x-axis.
    ///
    /// ```
    ///# use tilemap::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.width(), 2);
    /// ```
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the size along the y-axis.
    ///
    /// ```
    ///# use tilemap::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.height(), 3);
    /// ```
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Converts an index to the x-coordinate of the equivalent [`Point`].
    ///
    /// ```
    ///# use tilemap::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_x(5), 1);
    /// ```
    pub fn to_x(&self, index: usize) -> i32 {
        index as i32 % self.width as i32
    }

    /// Converts an index to the y-coordinate of the equivalent [`Point`].
    ///
    /// ```
    ///# use tilemap::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.to_y(5), 2);
    /// ```
    pub fn to_y(&self, index: usize) -> i32 {
        index as i32 / self.width as i32
    }

    /// Converts a [`Point`] to the equivalent index, but returns a wrong result if it is outside.
    ///
    /// ```
    ///# use tilemap::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// assert_eq!(size.convert_x_y(1, 2), 5);
    /// ```
    pub fn convert_x_y(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
}

/// Multiplies a [`Size2d`] with another.
///
/// ```
///# use tilemap::math::size2d::Size2d;
/// let a = Size2d::new(10, 30);
/// let b = Size2d::new(2, 5);
///
/// assert_eq!(a * b, Size2d::new(20, 150));
/// ```
impl Mul<Size2d> for Size2d {
    type Output = Self;

    fn mul(self, other: Size2d) -> Size2d {
        Size2d::new(self.width * other.width, self.height * other.height)
    }
}
