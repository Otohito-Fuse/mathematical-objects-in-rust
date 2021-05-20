use std::ops::{Add, AddAssign, Mul};
use crate::identities::{Zero,Identity};

/// （正方形とは限らない）行列
#[derive(Clone)]
pub struct Matrix<T> {
    entries: Vec<Vec<T>>,
    rows: usize,
    columns: usize,
}

/// 関連関数群。零元と単位元、加算と乗算を持っていることを仮定。
impl<T: Copy + Add<Output = T> + AddAssign<T> + Mul<Output = T> + Zero + Identity> Matrix<T> {
    pub fn new(r: usize, c: usize) -> Matrix<T> {
        Matrix {
            entries: vec![vec![T::zero(); c]; r],
            rows: r,
            columns: c,
        }
    }

    pub fn new_id(zero: T, one: T, n: usize) -> Matrix<T> {
        let mut v = vec![vec![zero; n]; n];
        for i in 0..n {
            v[i][i] = one;
        }
        Matrix {
            entries: v,
            rows: n,
            columns: n,
        }
    }

    pub fn change_entry(&mut self, r: usize, c: usize, n: T) {
        self.entries[r][c] = n;
    }

    pub fn add_to_entry(&mut self, r: usize, c: usize, n: T) {
        self.entries[r][c] += n;
    }

    pub fn mul(a: &Matrix<T>, b: &Matrix<T>, zero: T) -> Matrix<T> {
        if a.columns != b.rows {
            Matrix {
                entries: vec![vec![zero; 0]; 0],
                rows: 0,
                columns: 0,
            }
        } else {
            let mut v = vec![vec![zero; b.columns]; a.rows];
            for i in 0..(a.rows) {
                for j in 0..(b.columns) {
                    for k in 0..(a.columns) {
                        v[i][j] += a.entries[i][k] * b.entries[k][j];
                    }
                }
            }
            Matrix {
                entries: v,
                rows: a.rows,
                columns: b.columns,
            }
        }
    }

    pub fn mul_ch(&mut self, b: &Matrix<T>, zero: T) {
        if self.columns != b.rows {
            return;
        } else {
            let mut v = vec![vec![zero; b.columns]; self.rows];
            for i in 0..(self.rows) {
                for j in 0..(b.columns) {
                    for k in 0..(self.columns) {
                        v[i][j] += self.entries[i][k] * b.entries[k][j];
                    }
                }
            }
            self.entries = v;
        }
    }

    pub fn mat_pow(a: &Matrix<T>, p: u64, zero: T, one: T) -> Matrix<T> {
        if a.columns != a.rows {
            Matrix {
                entries: vec![vec![zero; 0]; 0],
                rows: 0,
                columns: 0,
            }
        } else {
            let n = a.rows;
            let mut ans = Matrix::new_id(zero, one, n);
            let mut now = a.clone();
            let mut res = p;
            while res != 0 {
                let now_copy = now.clone();
                if res & 1 != 0 {
                    ans.mul_ch(&now_copy, zero);
                }
                res >>= 1;
                now.mul_ch(&now_copy, zero);
            }
            Matrix {
                entries: ans.entries,
                rows: n,
                columns: n,
            }
        }
    }
}