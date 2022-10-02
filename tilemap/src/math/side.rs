use self::Side::*;
use core::fmt;
use std::slice::Iter;

/// Used for the 4 sides of a [`tile`](crate::tilemap::tile::Tile).
///
/// ```svgbob
///
///   +----------> x-axis
///   |
///   |             back
///   |      +---------------+
///   |      |               |
///   |      |               |
///   | left |     A tile    | right
///   |      |               |
///   |      |               |
///   |      +---------------+
///   |           front
///   v
/// y-axis
///
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Side {
    Back,
    Left,
    Front,
    Right,
}

impl Side {
    /// Iterates of all sides counter-clockwise.
    pub fn iterator() -> Iter<'static, Side> {
        static SIDES: [Side; 4] = [Back, Left, Front, Right];
        SIDES.iter()
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut iter = Side::iterator();

        assert_eq!(iter.next(), Some(&Back));
        assert_eq!(iter.next(), Some(&Left));
        assert_eq!(iter.next(), Some(&Front));
        assert_eq!(iter.next(), Some(&Right));
        assert_eq!(iter.next(), None);
    }
}
