use super::Dir;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Coord(pub isize, pub isize);

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<Coord> for isize {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        Coord(self * rhs.0, self * rhs.1)
    }
}

impl Add<Dir> for Coord {
    type Output = Coord;

    fn add(self, rhs: Dir) -> Self::Output {
        self + rhs.to_coord()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        assert_eq!(Coord(4, 6), Coord(1, 2) + Coord(3, 4));
        assert_eq!(Coord(3, 6), 3 * Coord(1, 2));
        assert_eq!(Coord(3, 5), Coord(4, 5) + Dir::N);
    }
}
