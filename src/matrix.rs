use std::{
    fmt::Display,
    iter::zip,
    ops::{Add, AddAssign, Index, IndexMut, Mul, Sub},
};

use num::Zero;

// use super::vec3d::Vec3d;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub mat: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
{
    #[inline]
    pub fn new() -> Self {
        Matrix::default()
    }

    pub fn elem_mult(&self, rhs: Self) -> Matrix<T, ROWS, COLS>
    where
        T: Mul<Output = T>,
    {
        let mut out = Matrix::default();
        for (i, (r1, r2)) in zip(self.mat, rhs.mat).enumerate() {
            for (j, (e1, e2)) in zip(r1, r2).enumerate() {
                out.mat[i][j] = e1 * e2;
            }
        }
        out
    }

    #[inline]
    pub fn get_size(&self) -> (usize, usize) {
        (ROWS, COLS)
    }

    pub fn transpose(&self) -> Matrix<T, COLS, ROWS> {
        let mut out = Matrix::new();
        for (i, r1) in self.mat.iter().enumerate() {
            for (j, e1) in r1.iter().enumerate() {
                out.mat[j][i] = *e1;
            }
        }
        out
    }

    pub fn reshape<const NEW_ROWS: usize, const NEW_COLS: usize>(
        &self,
    ) -> Matrix<T, NEW_ROWS, NEW_COLS> {
        let mut out = Matrix::new();
        for i in 0..NEW_ROWS {
            for j in 0..NEW_COLS {
                if j < COLS && i < ROWS {
                    out.mat[i][j] = self.mat[i][j];
                } else {
                    out.mat[i][j] = T::default();
                }
            }
        }
        out
    }
}

impl<T, const ROWS: usize, const COLS: usize> Display for Matrix<T, ROWS, COLS>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.mat {
            for e in row {
                f.write_fmt(format_args!("{} ", e))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl<T, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            mat: [[T::default(); COLS]; ROWS],
        }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Add for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Matrix<T, ROWS, COLS>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Matrix::default();

        for (i, (r1, r2)) in zip(self.mat, rhs.mat).enumerate() {
            for (j, (e1, e2)) in zip(r1, r2).enumerate() {
                out.mat[i][j] = e1 + e2;
            }
        }
        out
    }
}

impl<T, const ROWS: usize, const COLS: usize> Sub for Matrix<T, ROWS, COLS>
where
    T: Sub<Output = T> + Default + Copy,
{
    type Output = Matrix<T, ROWS, COLS>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = Matrix::default();

        for (i, (r1, r2)) in zip(self.mat, rhs.mat).enumerate() {
            for (j, (e1, e2)) in zip(r1, r2).enumerate() {
                out.mat[i][j] = e1 - e2;
            }
        }
        out
    }
}

impl<T, const ROWS: usize, const COLS: usize, const N: usize> Mul<Matrix<T, COLS, N>>
    for Matrix<T, ROWS, COLS>
where
    T: Default + Copy + Zero + Mul<Output = T>,
    T: AddAssign,
{
    type Output = Matrix<T, ROWS, N>;

    fn mul(self, rhs: Matrix<T, COLS, N>) -> Matrix<T, ROWS, N> {
        let mut out = Matrix::default();

        for (i, row) in out.mat.iter_mut().enumerate() {
            for (j, e) in row.iter_mut().enumerate() {
                *e = T::zero();

                for k in 0..COLS {
                    *e += self.mat[i][k] * rhs.mat[k][j];
                }
            }
        }
        out
    }
}

impl<T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]> for Matrix<T, ROWS, COLS> {
    fn from(value: [[T; COLS]; ROWS]) -> Self {
        Self { mat: value }
    }
}

// impl<T> From<Vec3d<T>> for Matrix<T, 1, 3> {
//     fn from(value: Vec3d<T>) -> Matrix<T, 1, 3> {
//         [value.into()].into()
//     }
// }

impl<T, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS> {
    type Output = [T; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.mat[index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<T, ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mat[index]
    }
}

// impl<T, const ROWS: usize, const COLS: usize> From<Vec3d<T>> for Matrix<T, ROWS, COLS>
// where
//     T: Default + Copy,
// {
//     fn from(value: Vec3d<T>) -> Self {
//         let vec_mat: Matrix<T, 1, 3> = Matrix {
//             mat: [[value.x, value.y, value.z]],
//         };
//         let mat_resh: Matrix<T, ROWS, COLS> = vec_mat.reshape();
//         mat_resh
//     }
// }
