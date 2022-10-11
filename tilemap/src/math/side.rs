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

    /// Do the 2 sides form a straight line at a node?
    pub fn is_straight(&self, other: Side) -> bool {
        match self {
            Back => other == Front,
            Left => other == Right,
            Front => other == Back,
            Right => other == Left,
        }
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

    #[test]
    fn test_is_straight() {
        assert!(Back.is_straight(Front));
        assert!(Left.is_straight(Right));
        assert!(Front.is_straight(Back));
        assert!(Right.is_straight(Left))
    }

    #[test]
    fn test_is_not_straight() {
        assert!(!Back.is_straight(Back));
        assert!(!Back.is_straight(Left));
        assert!(!Back.is_straight(Right));

        assert!(!Left.is_straight(Back));
        assert!(!Left.is_straight(Left));
        assert!(!Left.is_straight(Front));

        assert!(!Front.is_straight(Left));
        assert!(!Front.is_straight(Front));
        assert!(!Front.is_straight(Right));

        assert!(!Right.is_straight(Back));
        assert!(!Right.is_straight(Front));
        assert!(!Right.is_straight(Right));
    }
}
