use std::ops::{Add, AddAssign, Mul, MulAssign};

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

    pub fn iter_col_major(min: MatrixVec, max: MatrixVec) -> impl Iterator<Item = MatrixVec> {
        (min.col..max.col).flat_map(move |col| (min.row..max.row).map(move |row| (row, col).into()))
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

        let mut iter_col_major = MatrixVec::iter_col_major((1, 2).into(), (3, 4).into());

        assert_eq!(Some((1, 2).into()), iter_col_major.next());
        assert_eq!(Some((2, 2).into()), iter_col_major.next());
        assert_eq!(Some((1, 3).into()), iter_col_major.next());
        assert_eq!(Some((2, 3).into()), iter_col_major.next());
        assert_eq!(None, iter_col_major.next());
    }
}
