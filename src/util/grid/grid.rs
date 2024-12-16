use super::Coord;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T> {
    dim: Coord,
    vec: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_elem(dim: Coord, elem: T) -> Self
    where
        T: Clone,
    {
        if dim.0 < 0 || dim.1 < 0 {
            panic!("dim < 0");
        }

        let len = dim.0.checked_mul(dim.1).unwrap() as usize;
        Grid::<T> { dim, vec: vec![elem; len]}
    }

    pub fn from_vec(dim: Coord, vec: Vec<T>) -> Self {
        if dim.0 < 0 || dim.1 < 0 {
            panic!("dim < 0");
        }

        let len = dim.0.checked_mul(dim.1).unwrap() as usize;
        if vec.len() != len {
            panic!("vec.len() != dim.x * dim.y");
        }

        Grid::<T> { dim, vec }
    }

    pub fn from_str(s: &str) -> Self
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut vec = Vec::with_capacity(s.len());
        let mut char_buf = [0u8];

        let mut row = 0;
        let mut col = 0;

        for c in s.trim_end().chars() {
            if c == '\n' {
                row += 1;
                col = 0;
            } else {
                vec.push(T::from_str(c.encode_utf8(&mut char_buf)).unwrap());
                col += 1;
            }
        }

        Grid::from_vec(Coord(row + 1, col), vec)
    }

    pub fn dim(&self) -> Coord {
        self.dim
    }

    pub fn get(&self, idx: Coord) -> Option<&T> {
        if !self.is_in_bounds(idx) {
            return None;
        }

        Some(&self.vec[(idx.0 * self.dim.1 + idx.1) as usize])
    }

    pub fn get_mut(&mut self, idx: Coord) -> Option<&mut T> {
        if !self.is_in_bounds(idx) {
            return None;
        }

        Some(&mut self.vec[(idx.0 * self.dim.1 + idx.1) as usize])
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.vec.iter_mut()
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = (Coord, &T)> {
        (0..self.dim.0)
            .into_iter()
            .flat_map(|row| (0..self.dim.1).into_iter().map(move |col| Coord(row, col)))
            .zip(self.vec.iter())
    }

    fn is_in_bounds(&self, idx: Coord) -> bool {
        idx.0 >= 0 && idx.1 >= 0 && idx.0 < self.dim.0 && idx.1 < self.dim.1
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, idx: Coord) -> &Self::Output {
        self.get(idx).unwrap()
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, idx: Coord) -> &mut Self::Output {
        self.get_mut(idx).unwrap()
    }
}

/*impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dim.0 {
            for col in 0..self.dim.1 {
                self[Coord(row, col)].fmt(f)?
            }
            f.write_str("\n")?
        }

        Ok(())
    }
}*/

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dim.0 {
            for col in 0..self.dim.1 {
                if self[Coord(row, col)] {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }
            f.write_str("\n")?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_elem() {
        let grid = Grid::from_elem(Coord(2,3), 8);
        assert_eq!(8, grid[Coord(1,1)]);
        assert_eq!(8, grid[Coord(1,2)]);
    }

    #[test]
    fn test_from_str_and_index() {
        let grid = Grid::from_str("123\n456\n");

        assert_eq!(1, grid[Coord(0, 0)]);
        assert_eq!(2, grid[Coord(0, 1)]);
        assert_eq!(3, grid[Coord(0, 2)]);
        assert_eq!(4, grid[Coord(1, 0)]);
        assert_eq!(5, grid[Coord(1, 1)]);
        assert_eq!(6, grid[Coord(1, 2)]);

        assert_eq!(None, grid.get(Coord(-1, 0)));
        assert_eq!(None, grid.get(Coord(0, -1)));
        assert_eq!(Some(&1), grid.get(Coord(0, 0)));
        assert_eq!(Some(&2), grid.get(Coord(0, 1)));
        assert_eq!(Some(&3), grid.get(Coord(0, 2)));
        assert_eq!(None, grid.get(Coord(0, 3)));
        assert_eq!(None, grid.get(Coord(1, -1)));
        assert_eq!(Some(&4), grid.get(Coord(1, 0)));
        assert_eq!(Some(&5), grid.get(Coord(1, 1)));
        assert_eq!(Some(&6), grid.get(Coord(1, 2)));
        assert_eq!(None, grid.get(Coord(1, 3)));
        assert_eq!(None, grid.get(Coord(2, 0)));
    }

    #[test]
    fn test_from_vec_and_iter() {
        let grid = Grid::from_vec(Coord(2, 3), vec![1, 2, 3, 4, 5, 6]);

        let mut iter = grid.iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&4), iter.next());
        assert_eq!(Some(&5), iter.next());
        assert_eq!(Some(&6), iter.next());
        assert_eq!(None, iter.next());

        let mut indexed_iter = grid.indexed_iter();
        assert_eq!(Some((Coord(0, 0), &1)), indexed_iter.next());
        assert_eq!(Some((Coord(0, 1), &2)), indexed_iter.next());
        assert_eq!(Some((Coord(0, 2), &3)), indexed_iter.next());
        assert_eq!(Some((Coord(1, 0), &4)), indexed_iter.next());
        assert_eq!(Some((Coord(1, 1), &5)), indexed_iter.next());
        assert_eq!(Some((Coord(1, 2), &6)), indexed_iter.next());

        assert_eq!(None, iter.next());
    }
}
