use std::{
    cmp::Ordering,
    fmt,
    fmt::Formatter,
    ops::{Deref, DerefMut, Index, IndexMut, Mul, MulAssign},
};

use crate::fold_while;

use num_rational::Rational64 as R64;

/// A row vector (in the linear algebra sense) of rational numbers.
#[derive(Clone, Debug, PartialEq)]
pub struct RowVec(Vec<R64>);

impl RowVec {
    // TODO: test initializers
    pub fn new(v: Vec<R64>) -> Self {
        Self(v)
    }

    pub fn zeros(len: usize) -> Self {
        Self(vec![R64::ZERO; len])
    }

    pub fn from_int_vec(v: Vec<i64>) -> Self {
        Self(v.iter().map(|i| R64::from_integer(*i)).collect())
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Implements a "+=" operation (without actually defining the operator) which returns an error
    /// if the 2 `RowVec`s are of different sizes.
    pub fn add_assign(&mut self, rhs: &Self) -> anyhow::Result<()> {
        if self.0.len() != rhs.0.len() {
            anyhow::bail!(
                "addition of RowVecs of different sizes: {} vs {}",
                self.0.len(),
                rhs.0.len()
            );
        }

        self.0
            .iter_mut()
            .enumerate()
            .for_each(|(i, e)| *e += rhs.0[i]);

        Ok(())
    }

    /// Implements a "+" operation (without actually defining the operator) which returns an error
    /// if the 2 `RowVec`s are of different sizes.
    pub fn add(&self, rhs: &Self) -> anyhow::Result<Self> {
        let mut out = self.clone();
        out.add_assign(rhs)?;
        Ok(out)
    }

    /// Divides the `RowVec` by the `leader` (ie, the first non-zero entry), to make the leader `1`.
    /// Useful in matrix row reduction.
    pub fn normalize(&mut self) {
        let Some(leader_col) = self.leader_col() else {
            return;
        };

        if self.0[leader_col] != R64::ONE {
            let factor = self.0[leader_col].recip();
            *self *= factor;
        }
    }

    pub fn is_zeros(&self) -> bool {
        fold_while(self.0.iter(), true, |_, v| {
            let r = *v == R64::ZERO;
            (r, r)
        })
    }

    /// Returns the column (or index) of the first non-zero entry, or `None` if the `RowVec` is empty
    /// or all zero.
    pub fn leader_col(&self) -> Option<usize> {
        self.0.iter().position(|&e| e != R64::ZERO)
    }
}

impl Deref for RowVec {
    type Target = Vec<R64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RowVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MulAssign<R64> for RowVec {
    fn mul_assign(&mut self, rhs: R64) {
        self.0.iter_mut().for_each(|cell| *cell *= rhs);
    }
}

impl Mul<R64> for RowVec {
    type Output = RowVec;

    fn mul(mut self, rhs: R64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl fmt::Display for RowVec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        for v in &self.0 {
            write!(f, "{} ", *v)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

/// A matrix (in the linear algebra sense) of rational numbers.
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix(Vec<RowVec>);

impl Matrix {
    /// The caller is responsible for making sure the input is valid, meaning that no rows are
    /// empty, and each row is the same size. Failing to do so could lead to undefined behaviour or
    /// panics later on.
    pub fn new(m: Vec<Vec<R64>>) -> Self {
        Self(m.into_iter().map(RowVec).collect())
    }

    /// The caller is responsible for making sure the input is valid, meaning that no rows are
    /// empty, and each row is the same size. Failing to do so could lead to undefined behaviour or
    /// panics later on.
    pub fn from_row_vecs(m: Vec<RowVec>) -> Self {
        Self(m)
    }

    // TODO: test
    /// The caller is responsible for making sure the input is valid, meaning that no rows are
    /// empty, and each row is the same size. Failing to do so could lead to undefined behaviour or
    /// panics later on.
    pub fn from_int_vecs(m: Vec<Vec<i64>>) -> Self {
        Self(m.into_iter().map(RowVec::from_int_vec).collect())
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        if rows == 0 || cols == 0 {
            Self::empty()
        } else {
            Self(vec![RowVec::zeros(cols); rows])
        }
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Puts the matrix into Reduced Row Echelon Form.
    pub fn rref(&mut self) {
        self.r#ref();
        for row in 0..self.0.len() {
            self.eliminate_above_leader(row);
        }
        self.leader_sort();
        self.normalize();
    }

    // TODO(?) technically does more than necessary without the sort after each eliminate?
    /// Puts the matrix into (unreduced) Row Echelon Form.
    pub fn r#ref(&mut self) {
        self.leader_sort();
        for row in 0..self.0.len() {
            self.eliminate_below_leader(row);
        }
        self.leader_sort();
    }

    /// Sort the rows of the matrix such the ones with the leftmost leaders (ie, leftmost non-zero
    /// entries) come first. All-zero rows are at the bottom.
    ///
    /// ```
    /// use aoclib_rs::matrix::Matrix;
    ///
    /// let mut m = Matrix::from_int_vecs(vec![
    ///     vec![0, 2, 3, 4],
    ///     vec![2, 3, 6, 3],
    ///     vec![0, 0, 4, 5],
    ///     vec![0, 0, 0, 3],
    ///     vec![0, 5, 6, 3],
    ///     vec![0, 0, 0, 0],
    ///     vec![3, 4, 2, 6],
    /// ]);
    /// m.leader_sort();
    ///
    /// assert_eq!(
    ///     m,
    ///     Matrix::from_int_vecs(vec![
    ///         vec![2, 3, 6, 3],
    ///         vec![3, 4, 2, 6],
    ///         vec![0, 2, 3, 4],
    ///         vec![0, 5, 6, 3],
    ///         vec![0, 0, 4, 5],
    ///         vec![0, 0, 0, 3],
    ///         vec![0, 0, 0, 0],
    ///     ])
    /// );
    /// ```
    pub fn leader_sort(&mut self) {
        self.0.sort_by(|a, b| {
            for (i, ai) in a.iter().enumerate() {
                let bi = b[i];
                if *ai == R64::ZERO && bi != R64::ZERO {
                    return Ordering::Greater;
                } else if bi == R64::ZERO && *ai != R64::ZERO {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });
    }

    /// Perform a step in row reduction by eliminating the entries in the same column as `row`'s
    /// leader for all rows below `row`.
    pub fn eliminate_below_leader(&mut self, row: usize) {
        let Some(leader_col) = self.0[row].leader_col() else {
            return;
        };

        for i in (row + 1)..self.0.len() {
            self.eliminate(row, i, leader_col);
        }
    }

    /// Perform a step in row reduction by eliminating the `other_row` entry corresponding to
    /// `leader_col` (which is the column of the leader in `selected_row`).
    ///
    /// ```
    /// use aoclib_rs::matrix::Matrix;
    ///
    /// let mut m = Matrix::from_int_vecs(vec![vec![0, 2, 3, 4], vec![2, 4, 6, 3]]);
    /// m.eliminate(0, 1, 1);
    ///
    /// assert_eq!(
    ///     m,
    ///     Matrix::from_int_vecs(vec![vec![0, 2, 3, 4], vec![2, 0, 0, -5]])
    /// );
    /// ```
    pub fn eliminate(&mut self, selected_row: usize, other_row: usize, leader_col: usize) {
        if self.0[other_row][leader_col] == R64::ZERO {
            return;
        }

        let factor = -self.0[other_row][leader_col] / self.0[selected_row][leader_col];
        let term = self.0[selected_row].clone() * factor;
        self.0[other_row].add_assign(&term).unwrap();
    }

    /// Divides rows to ensure that each leader is `1`.
    pub fn normalize(&mut self) {
        self.0.iter_mut().for_each(|row| row.normalize());
    }

    /// Perform a step in row reduction by eliminating the entries in the same column as `row`'s
    /// leader for all rows above `row`.
    pub fn eliminate_above_leader(&mut self, row: usize) {
        let Some(leader_col) = self.0[row].leader_col() else {
            return;
        };

        for i in 0..row {
            self.eliminate(row, i, leader_col);
        }
    }

    /// Implements a "+=" operation (without actually defining the operator) which returns an error
    /// if the 2 `Matrix`es are of different sizes.
    pub fn add_assign(&mut self, rhs: &Self) -> anyhow::Result<()> {
        if self.0.len() != rhs.0.len() {
            anyhow::bail!(
                "addition of matrices of different heights: {} vs {}",
                self.0.len(),
                rhs.0.len()
            );
        }

        if self.0.is_empty() {
            return Ok(());
        }

        if self.0[0].len() != rhs.0[0].len() {
            anyhow::bail!(
                "addition of matrices of different widths: {} vs {}",
                self.0[0].len(),
                rhs.0[0].len()
            );
        }

        if self.0[0].is_empty() {
            return Ok(());
        }

        for i in 0..self.0.len() {
            self.0[i].add_assign(&rhs.0[i])?;
        }

        Ok(())
    }

    /// Implements a "+" operation (without actually defining the operator) which returns an error
    /// if the 2 `Matrix`es are of different sizes.
    pub fn add(&self, rhs: &Self) -> anyhow::Result<Self> {
        let mut out = self.clone();
        out.add_assign(rhs)?;
        Ok(out)
    }

    // TODO: test
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Implements matrix multiplication. Returns errors if the two matrices are incorrect sizes
    /// for multiplication.
    pub fn matrix_mul(&self, rhs: &Self) -> anyhow::Result<Self> {
        if self.is_empty() && rhs.is_empty() {
            return Ok(Self(Vec::new()));
        }

        if self.is_empty() {
            anyhow::bail!("multiplication of empty matrix with non-empty matrix");
        }

        if self.0[0].len() != rhs.0.len() {
            anyhow::bail!(
                "multiplication of incompatible matrices: lhs width {} vs rhs height {}",
                self.0[0].len(),
                rhs.0.len()
            );
        }

        let mut new = Self::zeros(self.0.len(), rhs.0[0].len());
        for i in 0..rhs.0[0].len() {
            for j in 0..self.0.len() {
                for k in 0..self.0[0].len() {
                    new.0[j][i] += self.0[j][k] * rhs.0[k][i];
                }
            }
        }
        Ok(new)
    }

    /// Appends a `RowVec` to the bottom of the matrix. Note that the caller is responsible for
    /// ensuring the RowVec is the correct size. Appending `RowVec`s of the wrong size could lead to
    /// undefined behaviour or panics later on.
    pub fn append_row(&mut self, r: RowVec) {
        self.0.push(r);
    }

    /// Remove row `i` from the `Matrix`.
    pub fn remove_row(&mut self, i: usize) {
        self.0.remove(i);
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    /// Returns an iterator over rows of the `Matrix`.
    pub fn iter(&self) -> impl Iterator<Item = &RowVec> {
        self.0.iter()
    }

    pub fn is_zeros(&self) -> bool {
        fold_while(self.0.iter(), true, |_, v| {
            let r = v.is_zeros();
            (r, r)
        })
    }

    /// Returns a new matrix with a single column whose contents are a copy of a column `c` from
    /// the original matrix.
    pub fn get_column_copy(&self, c: usize) -> Self {
        Self::new(self.iter().map(|r| vec![r[c]]).collect())
    }
}

impl MulAssign<R64> for Matrix {
    fn mul_assign(&mut self, rhs: R64) {
        self.0.iter_mut().for_each(|row| *row *= rhs);
    }
}

impl Mul<R64> for Matrix {
    type Output = Self;

    fn mul(mut self, rhs: R64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Index<usize> for Matrix {
    type Output = RowVec;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = R64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl IntoIterator for Matrix {
    type Item = RowVec;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        let mut first = true;
        for v in &self.0 {
            if !first {
                write!(f, "\n  ")?;
            }
            first = false;
            write!(f, "{} ", *v)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_vec_add_assign_good() {
        let mut rv1 = RowVec::from_int_vec(vec![1, 2, 3]);
        let rv2 = RowVec::from_int_vec(vec![4, 7, 9]);
        let r = rv1.add_assign(&rv2);
        assert!(r.is_ok());
        assert_eq!(rv1, RowVec::from_int_vec(vec![5, 9, 12]));
    }

    #[test]
    fn test_row_vec_add_assign_bad() {
        let mut rv1 = RowVec::from_int_vec(vec![1, 2, 3]);
        let rv2 = RowVec::from_int_vec(vec![4, 7, 9, 10]);
        let r = rv1.add_assign(&rv2);
        assert!(r.is_err());
        assert_eq!(rv1, RowVec::from_int_vec(vec![1, 2, 3]));
    }

    #[test]
    fn test_row_vec_add_assign_empty() {
        let mut rv1 = RowVec::empty();
        let rv2 = RowVec::empty();
        let r = rv1.add_assign(&rv2);
        assert!(r.is_ok());
        assert_eq!(rv1, RowVec::empty());
    }

    #[test]
    fn test_row_vec_add_good() {
        let rv1 = RowVec::from_int_vec(vec![1, 2, 3]);
        let rv2 = RowVec::from_int_vec(vec![4, 7, 9]);
        let r = rv1.add(&rv2);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), RowVec::from_int_vec(vec![5, 9, 12]));
    }

    #[test]
    fn test_row_vec_add_bad() {
        let rv1 = RowVec::from_int_vec(vec![1, 2, 3]);
        let rv2 = RowVec::from_int_vec(vec![4, 7, 9, 10]);
        let r = rv1.add(&rv2);
        assert!(r.is_err());
    }

    #[test]
    fn test_row_vec_add_empty() {
        let rv1 = RowVec::empty();
        let rv2 = RowVec::empty();
        let r = rv1.add(&rv2);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), RowVec(vec![]));
    }

    #[test]
    fn test_row_vec_normalize_good() {
        let mut rv = RowVec::from_int_vec(vec![7, 5, 9]);
        rv.normalize();
        assert_eq!(rv, RowVec(vec![R64::ONE, R64::new(5, 7), R64::new(9, 7)]));
    }

    #[test]
    fn test_row_vec_normalize_leading_zeros() {
        let mut rv = RowVec::from_int_vec(vec![0, 0, 7, 5, 9]);
        rv.normalize();
        assert_eq!(
            rv,
            RowVec(vec![
                R64::ZERO,
                R64::ZERO,
                R64::ONE,
                R64::new(5, 7),
                R64::new(9, 7)
            ])
        );
    }

    #[test]
    fn test_row_vec_normalize_leader_is_one() {
        let mut rv = RowVec::from_int_vec(vec![1, 2, 3]);
        rv.normalize();
        assert_eq!(rv, RowVec::from_int_vec(vec![1, 2, 3]));
    }

    #[test]
    fn test_row_vec_normalize_all_zeros() {
        let mut rv = RowVec::from_int_vec(vec![0, 0, 0]);
        rv.normalize();
        assert_eq!(rv, RowVec::from_int_vec(vec![0, 0, 0]));
    }

    #[test]
    fn test_row_vec_mul_assign_good() {
        let mut rv = RowVec::from_int_vec(vec![1, 2, 3]);
        rv *= R64::from_integer(4);
        assert_eq!(rv, RowVec::from_int_vec(vec![4, 8, 12]));
    }

    #[test]
    fn test_row_vec_mul_assign_empty() {
        let mut rv = RowVec::empty();
        rv *= R64::from_integer(4);
        assert_eq!(rv, RowVec::empty());
    }

    #[test]
    fn test_row_vec_mul_good() {
        let rv = RowVec::from_int_vec(vec![1, 2, 3]);
        assert_eq!(
            rv * R64::from_integer(4),
            RowVec::from_int_vec(vec![4, 8, 12])
        );
    }

    #[test]
    fn test_row_vec_mul_empty() {
        let rv = RowVec::empty();
        assert_eq!(rv * R64::from_integer(4), RowVec::from_int_vec(vec![]));
    }

    #[test]
    fn test_row_vec_is_zeros_false() {
        let rv = RowVec::from_int_vec(vec![0, 2, 0, 3, 0]);
        assert!(!rv.is_zeros());
    }

    #[test]
    fn test_row_vec_is_zeros_true() {
        let rv = RowVec::from_int_vec(vec![0, 0, 0, 0, 0]);
        assert!(rv.is_zeros());
    }

    #[test]
    fn test_row_vec_is_zeros_empty() {
        let rv = RowVec::empty();
        assert!(rv.is_zeros());
    }

    #[test]
    fn test_row_vec_leader_col_good() {
        let rv = RowVec::from_int_vec(vec![0, 0, 1, 2, 3]);
        assert_eq!(rv.leader_col(), Some(2));
    }

    #[test]
    fn test_row_vec_leader_col_first() {
        let rv = RowVec::from_int_vec(vec![4, 0, 1, 2, 3]);
        assert_eq!(rv.leader_col(), Some(0));
    }

    #[test]
    fn test_row_vec_leader_col_last() {
        let rv = RowVec::from_int_vec(vec![0, 0, 0, 0, 3]);
        assert_eq!(rv.leader_col(), Some(4));
    }

    #[test]
    fn test_row_vec_leader_col_zeros() {
        let rv = RowVec::from_int_vec(vec![0, 0, 0, 0, 0]);
        assert_eq!(rv.leader_col(), None);
    }

    #[test]
    fn test_row_vec_leader_col_empty() {
        let rv = RowVec::empty();
        assert_eq!(rv.leader_col(), None);
    }

    #[test]
    fn test_matrix_zeros() {
        let m = Matrix::zeros(4, 5);
        assert_eq!(
            m,
            Matrix::from_int_vecs(vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0]
            ])
        )
    }

    #[test]
    fn test_matrix_zeros_no_rows() {
        let m = Matrix::zeros(0, 5);
        assert_eq!(m, Matrix::new(Vec::new()));
    }

    #[test]
    fn test_matrix_zeros_no_cols() {
        let m = Matrix::zeros(3, 0);
        assert_eq!(m, Matrix::new(Vec::new()));
    }

    #[test]
    fn test_matrix_zeros_empty() {
        let m = Matrix::zeros(0, 0);
        assert_eq!(m, Matrix::new(Vec::new()));
    }

    #[test]
    fn test_matrix_leader_sort_good() {
        let mut m = Matrix::from_int_vecs(vec![
            vec![0, 0, 1, 10, 0],
            vec![5, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, -1, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
        ]);
        m.leader_sort();
        assert_eq!(
            m,
            Matrix::from_int_vecs(vec![
                vec![5, 0, 0, 0, 0],
                vec![0, -1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 0, 1, 10, 0],
                vec![0, 0, 0, 0, 0],
            ]),
        );
    }

    #[test]
    fn test_matrix_leader_sort_empty() {
        let mut m = Matrix::empty();
        m.leader_sort();
        assert_eq!(m, Matrix::empty());
    }

    #[test]
    fn test_matrix_eliminate_good() {
        let mut m = Matrix::from_int_vecs(vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ]);

        m.eliminate(2, 4, 0);
        assert_eq!(
            m,
            Matrix::from_row_vecs(vec![
                RowVec::from_int_vec(vec![1, 2, 3, 4, 5]),
                RowVec::from_int_vec(vec![6, 7, 8, 9, 10]),
                RowVec::from_int_vec(vec![11, 12, 13, 14, 15]),
                RowVec::from_int_vec(vec![16, 17, 18, 19, 20]),
                RowVec::new(vec![
                    R64::ZERO,
                    R64::new(-10, 11),
                    R64::new(-20, 11),
                    R64::new(-30, 11),
                    R64::new(-40, 11)
                ]),
            ])
        );

        // TODO: verify
        m.eliminate(4, 0, 1);
        assert_eq!(
            m,
            Matrix::from_row_vecs(vec![
                RowVec::from_int_vec(vec![1, 0, -1, -2, -3]),
                RowVec::from_int_vec(vec![6, 7, 8, 9, 10]),
                RowVec::from_int_vec(vec![11, 12, 13, 14, 15]),
                RowVec::from_int_vec(vec![16, 17, 18, 19, 20]),
                RowVec::new(vec![
                    R64::ZERO,
                    R64::new(-10, 11),
                    R64::new(-20, 11),
                    R64::new(-30, 11),
                    R64::new(-40, 11)
                ]),
            ])
        );

        // TODO: verify
        m.eliminate(2, 3, 2);
        assert_eq!(
            m,
            Matrix::from_row_vecs(vec![
                RowVec::from_int_vec(vec![1, 0, -1, -2, -3]),
                RowVec::from_int_vec(vec![6, 7, 8, 9, 10]),
                RowVec::from_int_vec(vec![11, 12, 13, 14, 15]),
                RowVec::new(vec![
                    R64::new(10, 13),
                    R64::new(5, 13),
                    R64::ZERO,
                    R64::new(-5, 13),
                    R64::new(-10, 13)
                ]),
                RowVec::new(vec![
                    R64::ZERO,
                    R64::new(-10, 11),
                    R64::new(-20, 11),
                    R64::new(-30, 11),
                    R64::new(-40, 11)
                ]),
            ])
        );
    }

    #[test]
    fn test_matrix_eliminate_other_already_eliminated() {
        let mut m = Matrix::from_int_vecs(vec![vec![0, 1, 2], vec![3, 0, 5]]);
        let m2 = m.clone();
        m.eliminate(0, 1, 1);
        assert_eq!(m, m2);
    }

    #[test]
    fn test_matrix_eliminate_self() {
        let mut m = Matrix::from_int_vecs(vec![vec![0, 1, 2]]);
        m.eliminate(0, 0, 1);
        assert_eq!(m, Matrix::zeros(1, 3));
    }

    #[test]
    #[should_panic]
    fn test_matrix_eliminate_leader_is_zero() {
        let mut m = Matrix::from_int_vecs(vec![vec![0, 1, 2], vec![3, 4, 5]]);
        m.eliminate(0, 1, 0);
    }

    #[test]
    fn test_matrix_normalize() {
        let mut m = Matrix::from_int_vecs(vec![
            vec![7, 5, 9],
            vec![0, 0, 7, 5, 9],
            vec![1, 2, 3],
            vec![0, 0, 0],
        ]);
        m.normalize();
        assert_eq!(
            m,
            Matrix::from_row_vecs(vec![
                RowVec::new(vec![R64::ONE, R64::new(5, 7), R64::new(9, 7)]),
                RowVec::new(vec![
                    R64::ZERO,
                    R64::ZERO,
                    R64::ONE,
                    R64::new(5, 7),
                    R64::new(9, 7)
                ]),
                RowVec::from_int_vec(vec![1, 2, 3]),
                RowVec::from_int_vec(vec![0, 0, 0]),
            ])
        );
    }

    #[test]
    fn test_matrix_add_assign_good() -> Result<(), Box<dyn std::error::Error>> {
        let mut m1 = Matrix::from_int_vecs(vec![vec![3, 5, 7], vec![8, 2, 3], vec![9, 2, 3]]);
        let m2 = Matrix::from_int_vecs(vec![vec![8, 2, 4], vec![1, 6, 7], vec![2, 3, 5]]);
        m1.add_assign(&m2)?;
        assert_eq!(
            m1,
            Matrix::from_int_vecs(vec![vec![11, 7, 11], vec![9, 8, 10], vec![11, 5, 8]])
        );
        Ok(())
    }

    #[test]
    fn test_matrix_add_assign_both_empty() -> Result<(), Box<dyn std::error::Error>> {
        let mut m1 = Matrix::empty();
        let m2 = Matrix::empty();
        m1.add_assign(&m2)?;
        assert_eq!(m1, Matrix::empty());
        Ok(())
    }

    #[test]
    fn test_matrix_add_assign_left_empty() {
        let mut m1 = Matrix::empty();
        let m1_copy = m1.clone();
        let m2 = Matrix::from_int_vecs(vec![vec![8, 2, 4], vec![1, 6, 7]]);
        assert!(m1.add_assign(&m2).is_err());
        assert_eq!(m1, m1_copy);
    }

    #[test]
    fn test_matrix_add_assign_right_empty() {
        let mut m1 = Matrix::from_int_vecs(vec![vec![3, 5, 7], vec![8, 2, 3], vec![9, 2, 3]]);
        let m1_copy = m1.clone();
        let m2 = Matrix::empty();
        assert!(m1.add_assign(&m2).is_err());
        assert_eq!(m1, m1_copy);
    }

    #[test]
    fn test_matrix_add_assign_row_mismatch() {
        let mut m1 = Matrix::from_int_vecs(vec![vec![3, 5, 7], vec![8, 2, 3], vec![9, 2, 3]]);
        let m1_copy = m1.clone();
        let m2 = Matrix::from_int_vecs(vec![vec![8, 2, 4], vec![1, 6, 7]]);
        assert!(m1.add_assign(&m2).is_err());
        assert_eq!(m1, m1_copy);
    }

    #[test]
    fn test_matrix_add_assign_col_mismatch() {
        let mut m1 = Matrix::from_int_vecs(vec![vec![3, 5], vec![8, 2]]);
        let m1_copy = m1.clone();
        let m2 = Matrix::from_int_vecs(vec![vec![8, 2, 4], vec![1, 6, 7]]);
        assert!(m1.add_assign(&m2).is_err());
        assert_eq!(m1, m1_copy);
    }

    #[test]
    fn test_matrix_add_good() -> Result<(), Box<dyn std::error::Error>> {
        let m1 = Matrix::from_int_vecs(vec![vec![3, 5, 7], vec![8, 2, 3], vec![9, 2, 3]]);
        let m2 = Matrix::from_int_vecs(vec![vec![8, 2, 4], vec![1, 6, 7], vec![2, 3, 5]]);
        let result = m1.add(&m2)?;
        assert_eq!(
            result,
            Matrix::from_int_vecs(vec![vec![11, 7, 11], vec![9, 8, 10], vec![11, 5, 8]])
        );
        Ok(())
    }

    #[test]
    fn test_matrix_add_empty() -> Result<(), Box<dyn std::error::Error>> {
        let m1 = Matrix::empty();
        let m2 = Matrix::empty();
        let result = m1.add(&m2)?;
        assert_eq!(result, Matrix::empty());
        Ok(())
    }

    #[test]
    fn test_matrix_add_row_mismatch() {
        let m1 = Matrix::from_int_vecs(vec![vec![3, 5, 7], vec![8, 2, 3], vec![9, 2, 3]]);
        let m2 = Matrix::from_int_vecs(vec![vec![8, 2, 4], vec![1, 6, 7]]);
        assert!(m1.add(&m2).is_err());
    }

    #[test]
    fn test_matrix_add_col_mismatch() {
        let m1 = Matrix::from_int_vecs(vec![vec![3, 5], vec![8, 2]]);
        let m2 = Matrix::from_int_vecs(vec![vec![8, 2, 4], vec![1, 6, 7]]);
        assert!(m1.add(&m2).is_err());
    }

    #[test]
    fn test_matrix_matrix_mul_good() -> Result<(), Box<dyn std::error::Error>> {
        let m1 = Matrix::from_int_vecs(vec![vec![3, 4], vec![7, 2], vec![1, 5]]);
        let m2 = Matrix::from_int_vecs(vec![vec![5, 6, 2, 3], vec![8, 3, 9, 7]]);
        let result = m1.matrix_mul(&m2)?;
        assert_eq!(
            result,
            Matrix::from_int_vecs(vec![
                vec![47, 30, 42, 37],
                vec![51, 48, 32, 35],
                vec![45, 21, 47, 38],
            ])
        );
        Ok(())
    }

    #[test]
    fn test_matrix_matrix_mul_both_empty() -> Result<(), Box<dyn std::error::Error>> {
        let m1 = Matrix::empty();
        let m2 = Matrix::empty();
        let result = m1.matrix_mul(&m2)?;
        assert_eq!(result, Matrix::empty());
        Ok(())
    }

    #[test]
    fn test_matrix_matrix_mul_left_empty() {
        let m1 = Matrix::empty();
        let m2 = Matrix::zeros(3, 4);
        assert!(m1.matrix_mul(&m2).is_err());
    }

    #[test]
    fn test_matrix_matrix_mul_right_empty() {
        let m1 = Matrix::zeros(2, 3);
        let m2 = Matrix::empty();
        assert!(m1.matrix_mul(&m2).is_err());
    }

    #[test]
    fn test_matrix_matrix_mul_mismatch() {
        let m1 = Matrix::zeros(3, 3);
        let m2 = Matrix::zeros(4, 4);
        assert!(m1.matrix_mul(&m2).is_err());
    }

    #[test]
    fn test_matrix_is_empty_true() {
        assert!(Matrix::empty().is_empty());
    }

    #[test]
    fn test_matrix_is_empty_false() {
        assert!(!Matrix::zeros(1, 1).is_empty());
    }

    #[test]
    fn test_matrix_height_good() {
        assert_eq!(Matrix::zeros(2, 3).height(), 2);
    }

    #[test]
    fn test_matrix_height_empty() {
        assert_eq!(Matrix::empty().height(), 0);
    }

    #[test]
    fn test_matrix_width_good() {
        assert_eq!(Matrix::zeros(2, 3).width(), 3);
    }

    #[test]
    fn test_matrix_width_empty() {
        assert_eq!(Matrix::empty().width(), 0);
    }

    #[test]
    fn test_matrix_append_row_good() {
        let mut m = Matrix::zeros(2, 3);
        let v = RowVec::zeros(3);
        m.append_row(v);
        assert_eq!(m, Matrix::zeros(3, 3));
    }

    #[test]
    fn test_matrix_append_row_empty() {
        let mut m = Matrix::empty();
        let v = RowVec::zeros(3);
        m.append_row(v);
        assert_eq!(m, Matrix::zeros(1, 3));
    }

    #[test]
    fn test_matrix_remove_row_good() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m.remove_row(1);
        assert_eq!(m, Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![7, 8, 9]]));
    }

    #[test]
    fn test_matrix_remove_row_first() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m.remove_row(0);
        assert_eq!(m, Matrix::from_int_vecs(vec![vec![4, 5, 6], vec![7, 8, 9]]));
    }

    #[test]
    fn test_matrix_remove_row_last() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m.remove_row(2);
        assert_eq!(m, Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6]]));
    }

    #[test]
    fn test_matrix_remove_row_only() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3]]);
        m.remove_row(0);
        assert_eq!(m, Matrix::empty());
    }

    #[test]
    fn test_matrix_is_zeros_true() {
        assert!(Matrix::zeros(2, 3).is_zeros());
    }

    #[test]
    fn test_matrix_is_zeros_false() {
        assert!(!Matrix::from_int_vecs(vec![vec![1]]).is_zeros());
    }

    #[test]
    fn test_matrix_is_zeros_empty() {
        assert!(Matrix::empty().is_zeros());
    }

    #[test]
    fn test_matrix_get_column_copy_good() {
        let m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(
            m.get_column_copy(1),
            Matrix::from_int_vecs(vec![vec![2], vec![5], vec![8]])
        );
    }

    #[test]
    fn test_matrix_get_column_copy_first() {
        let m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(
            m.get_column_copy(0),
            Matrix::from_int_vecs(vec![vec![1], vec![4], vec![7]])
        );
    }

    #[test]
    fn test_matrix_get_column_copy_last() {
        let m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(
            m.get_column_copy(2),
            Matrix::from_int_vecs(vec![vec![3], vec![6], vec![9]])
        );
    }

    #[test]
    fn test_matrix_get_column_copy_only() {
        let m = Matrix::from_int_vecs(vec![vec![3], vec![6], vec![9]]);
        assert_eq!(
            m.get_column_copy(0),
            Matrix::from_int_vecs(vec![vec![3], vec![6], vec![9]])
        );
    }

    #[test]
    fn test_matrix_eliminate_below_leader_good() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m.eliminate_below_leader(1);

        // TODO: verify
        assert_eq!(
            m,
            Matrix::from_row_vecs(vec![
                RowVec::from_int_vec(vec![1, 2, 3]),
                RowVec::from_int_vec(vec![4, 5, 6]),
                RowVec::new(vec![R64::ZERO, R64::new(-3, 4), R64::new(-3, 2)]),
            ])
        );
    }

    #[test]
    fn test_matrix_eliminate_below_leader_first() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m.eliminate_below_leader(0);

        // TODO: verify
        assert_eq!(
            m,
            Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![0, -3, -6], vec![0, -6, -12]])
        );
    }

    #[test]
    fn test_matrix_eliminate_below_leader_last() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let m_copy = m.clone();
        m.eliminate_below_leader(2);
        assert_eq!(m, m_copy);
    }

    #[test]
    fn test_matrix_eliminate_above_leader_good() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m.eliminate_above_leader(1);

        // TODO: verify
        assert_eq!(
            m,
            Matrix::from_row_vecs(vec![
                RowVec::new(vec![R64::ZERO, R64::new(3, 4), R64::new(3, 2)]),
                RowVec::from_int_vec(vec![4, 5, 6]),
                RowVec::from_int_vec(vec![7, 8, 9]),
            ])
        );
    }

    #[test]
    fn test_matrix_eliminate_above_leader_first() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let m_copy = m.clone();
        m.eliminate_above_leader(0);
        assert_eq!(m, m_copy);
    }

    #[test]
    fn test_matrix_eliminate_above_leader_last() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m.eliminate_above_leader(2);

        // TODO: verify
        assert_eq!(
            m,
            Matrix::from_row_vecs(vec![
                RowVec::new(vec![R64::ZERO, R64::new(6, 7), R64::new(12, 7)]),
                RowVec::new(vec![R64::ZERO, R64::new(3, 7), R64::new(6, 7)]),
                RowVec::from_int_vec(vec![7, 8, 9]),
            ])
        );
    }

    #[test]
    fn test_matrix_ref_good() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m.r#ref();
        assert_eq!(
            m,
            Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![0, -3, -6], vec![0, 0, 0]])
        );
    }

    #[test]
    fn test_matrix_ref_empty() {
        let mut m = Matrix::empty();
        m.r#ref();
        assert_eq!(m, Matrix::empty());
    }

    // TODO: more ref tests?

    #[test]
    fn test_matrix_rref_good() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m.rref();
        assert_eq!(
            m,
            Matrix::from_int_vecs(vec![vec![1, 0, -1], vec![0, 1, 2], vec![0, 0, 0]])
        );
    }

    #[test]
    fn test_matrix_rref_empty() {
        let mut m = Matrix::empty();
        m.rref();
        assert_eq!(m, Matrix::empty());
    }

    // TODO: more rref tests?

    #[test]
    fn test_matrix_mul_assign_good() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m *= R64::from_integer(5);
        assert_eq!(
            m,
            Matrix::from_int_vecs(vec![vec![5, 10, 15], vec![20, 25, 30], vec![35, 40, 45]])
        );
    }

    #[test]
    fn test_matrix_mul_assign_empty() {
        let mut m = Matrix::empty();
        m *= R64::from_integer(5);
        assert_eq!(m, Matrix::empty());
    }

    #[test]
    fn test_matrix_mul_good() {
        let m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let result = m * R64::from_integer(5);
        assert_eq!(
            result,
            Matrix::from_int_vecs(vec![vec![5, 10, 15], vec![20, 25, 30], vec![35, 40, 45]])
        );
    }

    #[test]
    fn test_matrix_mul_empty() {
        let m = Matrix::empty();
        let result = m * R64::from_integer(5);
        assert_eq!(result, Matrix::empty());
    }

    #[test]
    fn test_matrix_index() {
        assert_eq!(
            Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])[(1, 2)],
            R64::from_integer(6)
        );
    }

    #[test]
    fn test_matrix_index_mut() {
        let mut m = Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        m[(1, 2)] = R64::ZERO;
        assert_eq!(
            m,
            Matrix::from_int_vecs(vec![vec![1, 2, 3], vec![4, 5, 0], vec![7, 8, 9]])
        );
    }
}
