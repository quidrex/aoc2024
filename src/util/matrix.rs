use crate::util::matrix_vec::MatrixVec;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Matrix<T> {
    dim: MatrixVec,
    vec: Vec<T>,
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
