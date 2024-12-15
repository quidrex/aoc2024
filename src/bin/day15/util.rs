use ndarray::Array2;
use std::ops::Add;
use strum::{EnumIter, FromRepr};

pub type Position = [usize; 2];

pub fn parse_matrix<T, F>(input: &str, mut f: F) -> Array2<T>
where
    F: FnMut(Position, char) -> T,
{
    let mut vec = Vec::with_capacity(input.len());

    let mut row = 0;
    let mut col = 0;

    for c in input.trim_end().chars() {
        if c == '\n' {
            row += 1;
            col = 0;
        } else {
            vec.push(f([row, col], c));
            col += 1;
        }
    }

    Array2::from_shape_vec((row + 1, col), vec).unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromRepr, EnumIter)]
pub enum Dir {
    U = 0,
    R,
    D,
    L,
}

impl Dir {
    pub fn dir(&self) -> [isize; 2] {
        match self {
            Dir::U => [-1, 0],
            Dir::R => [0, 1],
            Dir::D => [1, 0],
            Dir::L => [0, -1],
        }
    }
}

impl Add<Dir> for [usize; 2] {
    type Output = [usize; 2];

    fn add(self, rhs: Dir) -> Self::Output {
        let dir = rhs.dir();
        [
            self[0].wrapping_add_signed(dir[0]),
            self[1].wrapping_add_signed(dir[1]),
        ]
    }
}
