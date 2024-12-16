use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Clone)]
pub struct Matrix<T> {
    dim: MatrixVec,
    vec: Vec<T>,
}

#[allow(dead_code)]
impl<T> Matrix<T> {
    pub fn from_vec(dim: MatrixVec, vec: Vec<T>) -> Matrix<T> {
        if dim.row < 0 || dim.col < 0 {
            panic!("dim < 0");
        }

        let len = dim.row.checked_mul(dim.col).unwrap() as usize;
        if vec.len() != len {
            panic!("vec.len() != dim.x * dim.y");
        }

        Matrix::<T> { dim, vec }
    }

    pub fn dim(&self) -> MatrixVec {
        self.dim
    }

    pub fn get(&self, idx: MatrixVec) -> Option<&T> {
        if idx.row < 0 || idx.col < 0 || idx.row >= self.dim.row || idx.col >= self.dim.col {
            return None;
        }

        Some(&self.vec[(idx.row * self.dim.col + idx.col) as usize])
    }

    pub fn get_mut(&mut self, idx: MatrixVec) -> Option<&mut T> {
        if idx.row < 0 || idx.col < 0 || idx.row >= self.dim.row || idx.col >= self.dim.col {
            return None;
        }

        Some(&mut self.vec[(idx.row * self.dim.col + idx.col) as usize])
    }
}

impl<T: Clone> Matrix<T> {
    pub fn from_elem(dim: MatrixVec, elem: T) -> Matrix<T> {
        if dim.row < 0 || dim.col < 0 {
            panic!("dim < 0");
        }

        let len = dim.row.checked_mul(dim.col).unwrap() as usize;
        let vec = vec![elem; len];

        Matrix::<T> { dim, vec }
    }
}

impl<T> Index<MatrixVec> for Matrix<T> {
    type Output = T;

    fn index(&self, index: MatrixVec) -> &T {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<MatrixVec> for Matrix<T> {
    fn index_mut(&mut self, index: MatrixVec) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct MatrixVec {
    pub row: i32,
    pub col: i32,
}

impl From<(i32, i32)> for MatrixVec {
    fn from(value: (i32, i32)) -> MatrixVec {
        MatrixVec {
            row: value.0,
            col: value.1,
        }
    }
}

impl MatrixVec {
    pub fn mod_euc(self, rhs: MatrixVec) -> MatrixVec {
        MatrixVec {
            row: ((self.row % rhs.row) + rhs.row) % rhs.row,
            col: ((self.col % rhs.col) + rhs.col) % rhs.col,
        }
    }

    pub fn iter_row_major(min: MatrixVec, max: MatrixVec) -> impl Iterator<Item = MatrixVec> {
        (min.row..max.row).flat_map(move |row| (min.col..max.col).map(move |col| (row, col).into()))
    }
}

impl Add for MatrixVec {
    type Output = MatrixVec;

    fn add(self, rhs: MatrixVec) -> MatrixVec {
        MatrixVec {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl AddAssign for MatrixVec {
    fn add_assign(&mut self, rhs: Self) {
        self.row = self.row + rhs.row;
        self.col = self.col + rhs.col;
    }
}

impl Mul<i32> for MatrixVec {
    type Output = MatrixVec;

    fn mul(self, rhs: i32) -> MatrixVec {
        MatrixVec {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

impl MulAssign<i32> for MatrixVec {
    fn mul_assign(&mut self, rhs: i32) {
        self.row *= rhs;
        self.col *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_ops() {
        assert_eq!(
            MatrixVec::from((4, 6)),
            MatrixVec::from((1, 2)) + MatrixVec::from((3, 4))
        );
        assert_eq!(MatrixVec::from((3, 6)), MatrixVec::from((1, 2)) * 3);
        assert_eq!(
            MatrixVec::from((5, 1)),
            MatrixVec::from((-1, 8)).mod_euc(MatrixVec::from((6, 7)))
        );

        let mut lhs_add = MatrixVec::from((1, 2));
        lhs_add += MatrixVec::from((3, 4));
        assert_eq!(MatrixVec::from((4, 6)), lhs_add);

        let mut lhs_mul = MatrixVec::from((1, 2));
        lhs_mul *= 3;
        assert_eq!(MatrixVec::from((3, 6)), lhs_mul);
    }

    #[test]
    fn test_success_iter() {
        let mut iter_row_major = MatrixVec::iter_row_major((1, 2).into(), (3, 4).into());

        assert_eq!(Some((1, 2).into()), iter_row_major.next());
        assert_eq!(Some((1, 3).into()), iter_row_major.next());
        assert_eq!(Some((2, 2).into()), iter_row_major.next());
        assert_eq!(Some((2, 3).into()), iter_row_major.next());
        assert_eq!(None, iter_row_major.next());
    }

    #[test]
    fn test_success_from_vec() {
        let vec = vec![1, 2, 3, 4, 5, 6];
        let mut matrix = Matrix::from_vec((2, 3).into(), vec);

        assert_eq!(MatrixVec::from((2, 3)), matrix.dim());
        assert_eq!(None, matrix.get((-1, 0).into()));
        assert_eq!(None, matrix.get((0, -1).into()));
        assert_eq!(None, matrix.get((2, 0).into()));
        assert_eq!(None, matrix.get((0, 3).into()));

        if let Some(elem) = matrix.get_mut((0, 0).into()) {
            *elem = 8;
        };
        matrix[(0, 1).into()] = 9;

        assert_eq!(Some(&8), matrix.get((0, 0).into()));
        assert_eq!(Some(&9), matrix.get((0, 1).into()));
        assert_eq!(Some(&3), matrix.get((0, 2).into()));
        assert_eq!(Some(&4), matrix.get((1, 0).into()));
        assert_eq!(Some(&5), matrix.get((1, 1).into()));
        assert_eq!(Some(&6), matrix.get((1, 2).into()));

        assert_eq!(6, matrix[(1, 2).into()]);
    }

    #[test]
    fn test_success_from_elem() {
        let mut matrix = Matrix::from_elem((2, 3).into(), 7);

        if let Some(elem) = matrix.get_mut((0, 0).into()) {
            *elem = 8;
        };
        matrix[(0, 1).into()] = 9;

        assert_eq!(MatrixVec::from((2, 3)), matrix.dim());
        assert_eq!(None, matrix.get((-1, 0).into()));
        assert_eq!(None, matrix.get((0, -1).into()));
        assert_eq!(None, matrix.get((2, 0).into()));
        assert_eq!(None, matrix.get((0, 3).into()));

        assert_eq!(Some(&8), matrix.get((0, 0).into()));
        assert_eq!(Some(&9), matrix.get((0, 1).into()));
        assert_eq!(Some(&7), matrix.get((0, 2).into()));
        assert_eq!(Some(&7), matrix.get((1, 0).into()));
        assert_eq!(Some(&7), matrix.get((1, 1).into()));
        assert_eq!(Some(&7), matrix.get((1, 2).into()));

        assert_eq!(7, matrix[(1, 2).into()]);
    }
}
