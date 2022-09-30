use crate::math::size2d::Size2d;
use std::ops::{Add, Div, Mul, Sub};

#[svgbobdoc::transform]
/// Defines a point in 2 dimensions.
///
/// # Diagram
///
/// ```svgbob
///      0  1
///   +--*--*----> x-axis
///   |
/// 0 *
///   |
/// 1 *
///   |
/// 2 *     * (1,2)
///   |
///   v
/// y-axis
/// ```
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

impl Point2d {
    /// Returns a new point.
    ///
    /// ```
    ///# use tilemap::math::point2d::Point2d;
    /// let point = Point2d::new(2, 3);
    /// assert_eq!(point.x, 2);
    /// assert_eq!(point.y, 3);
    /// ```
    pub const fn new(x: i32, y: i32) -> Point2d {
        Point2d { x, y }
    }

    /// Calculates the euclidean distance to another point.
    ///
    /// ```
    ///# use tilemap::math::point2d::Point2d;
    /// let a = Point2d::new(1, 2);
    /// let b = Point2d::new(4, 6);
    ///
    /// assert_eq!(a.calculate_distance(&a), 0.0);
    /// assert_eq!(a.calculate_distance(&b), 5.0);
    /// assert_eq!(b.calculate_distance(&a), 5.0);
    /// ```
    pub fn calculate_distance(&self, point: &Point2d) -> f32 {
        (self.x as f32 - point.x as f32).hypot(self.y as f32 - point.y as f32)
    }
}

/// Add a [`Point2d`] to another [`Point2d`].
///
/// ```
///# use tilemap::math::point2d::Point2d;
/// let a = Point2d::new(1, 2);
/// let b = Point2d::new(30, 50);
/// let result = Point2d::new(31, 52);
///
/// assert_eq!(a + b, result);
/// assert_eq!(b + a, result);
/// ```
impl Add<Point2d> for Point2d {
    type Output = Point2d;

    fn add(self, other: Point2d) -> Point2d {
        Point2d::new(self.x + other.x, self.y + other.y)
    }
}

/// Adds an integer to a [`Point2d`].
///
/// ```
///# use tilemap::math::point2d::Point2d;
/// let point = Point2d::new(1, 2);
///
/// assert_eq!(point + 2, Point2d::new(3, 4));
/// ```
impl Add<i32> for Point2d {
    type Output = Point2d;

    fn add(self, value: i32) -> Point2d {
        Point2d::new(self.x + value, self.y + value)
    }
}

/// Adds a [`Size2d`] to a [`Point2d`].
///
/// ```
///# use tilemap::math::point2d::Point2d;
///# use tilemap::math::size2d::Size2d;
/// let point = Point2d::new(1, 2);
/// let size = Size2d::new(30, 50);
///
/// assert_eq!(point + size, Point2d::new(31, 52));
/// ```
impl Add<Size2d> for Point2d {
    type Output = Point2d;

    fn add(self, size: Size2d) -> Point2d {
        Point2d::new(self.x + size.width() as i32, self.y + size.height() as i32)
    }
}

/// Subtracts an integer from a [`Point2d`].
///
/// ```
///# use tilemap::math::point2d::Point2d;
/// let point = Point2d::new(1, 2);
///
/// assert_eq!(point - 2, Point2d::new(-1, 0));
/// ```
impl Sub<i32> for Point2d {
    type Output = Point2d;

    fn sub(self, value: i32) -> Point2d {
        Point2d::new(self.x - value, self.y - value)
    }
}

/// Subtracts a [`Point2d`] from another [`Point2d`].
///
/// ```
///# use tilemap::math::point2d::Point2d;
/// let a = Point2d::new(1, 2);
/// let b = Point2d::new(30, 50);
///
/// assert_eq!(a - b, Point2d::new(-29, -48));
/// assert_eq!(b - a, Point2d::new(29, 48));
/// ```
impl Sub<Point2d> for Point2d {
    type Output = Point2d;

    fn sub(self, other: Point2d) -> Point2d {
        Point2d::new(self.x - other.x, self.y - other.y)
    }
}

/// Subtracts a [`Size2d`] from a [`Point2d`].
///
/// ```
///# use tilemap::math::point2d::Point2d;
///# use tilemap::math::size2d::Size2d;
/// let point = Point2d::new(1, 2);
/// let size = Size2d::new(30, 50);
///
/// assert_eq!(point - size, Point2d::new(-29, -48));
/// ```
impl Sub<Size2d> for Point2d {
    type Output = Point2d;

    fn sub(self, size: Size2d) -> Point2d {
        Point2d::new(self.x - size.width() as i32, self.y - size.height() as i32)
    }
}

/// Multiplies a [`Point2d`] by an integer.
///
/// ```
///# use tilemap::math::point2d::Point2d;
/// let point = Point2d::new(10, 30);
///
/// assert_eq!(point * 2, Point2d::new(20, 60));
/// ```
impl Mul<u32> for Point2d {
    type Output = Self;

    fn mul(self, value: u32) -> Self::Output {
        Point2d::new(self.x * value as i32, self.y * value as i32)
    }
}

/// Multiplies a [`Point2d`] by a [`Size2d`].
///
/// ```
///# use tilemap::math::point2d::Point2d;
///# use tilemap::math::size2d::Size2d;
/// let point = Point2d::new(10, 30);
/// let size = Size2d::new(20, 40);
///
/// assert_eq!(point * size, Point2d::new(200, 1200));
/// ```
impl Mul<Size2d> for Point2d {
    type Output = Self;

    fn mul(self, size: Size2d) -> Self::Output {
        Point2d::new(self.x * size.width() as i32, self.y * size.height() as i32)
    }
}

/// Divides a [`Point2d`] by an integer.
///
/// ```
///# use tilemap::math::point2d::Point2d;
/// let point = Point2d::new(10, -30);
///
/// assert_eq!(point / 2, Point2d::new(5, -15));
/// ```
impl Div<u32> for Point2d {
    type Output = Self;

    fn div(self, value: u32) -> Self::Output {
        Point2d::new(self.x / value as i32, self.y / value as i32)
    }
}
