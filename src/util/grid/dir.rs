use super::Coord;
use strum::{EnumIter, FromRepr};

#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumIter, FromRepr)]
pub enum Dir {
    N = 0,
    E,
    S,
    W,
}

impl Dir {
    pub fn to_coord(&self) -> Coord {
        match self {
            Dir::N => Coord(-1, 0),
            Dir::E => Coord(0, 1),
            Dir::S => Coord(1, 0),
            Dir::W => Coord(0, -1),
        }
    }

    pub fn turn(&self, times: isize) -> Dir {
        Dir::from_repr((((*self as isize + times) % 4 + 4) % 4) as usize).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_coord() {
        assert_eq!(Coord(1, 0), Dir::S.to_coord());
    }

    #[test]
    fn test_turn() {
        assert_eq!(Dir::W, Dir::N.turn(-1));
        assert_eq!(Dir::N, Dir::N.turn(-0));
        assert_eq!(Dir::E, Dir::N.turn(1));
        assert_eq!(Dir::S, Dir::N.turn(2));
        assert_eq!(Dir::S, Dir::N.turn(6));
    }
}
