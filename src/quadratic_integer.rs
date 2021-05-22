use crate::identities::{Identity, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Z\[x\]/(x^2 - Bx - C) の元
///
/// B = 0, C = -1 のときは Z\[i\] など
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct QuadInt<const B: i64, const C: i64> {
    constant: i64,
    first: i64,
}

impl<const B: i64, const C: i64> QuadInt<B, C> {
    /// コンストラクタ。1つめの引数が定数項。2つめが1次の項。
    pub fn new(constant: i64, first: i64) -> Self {
        Self {
            constant: constant,
            first: first,
        }
    }
}

/// ```println!```などで見やすく表示させるため、```Display```トレイトを実装。
impl<const B: i64, const C: i64> fmt::Display for QuadInt<B, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} + {}x]", self.constant, self.first)
    }
}

impl<const B: i64, const C: i64> Add for QuadInt<B, C> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            constant: self.constant + rhs.constant,
            first: self.first + rhs.first,
        }
    }
}

impl<const B: i64, const C: i64> AddAssign for QuadInt<B, C> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            constant: self.constant + other.constant,
            first: self.first + other.first,
        }
    }
}

impl<const B: i64, const C: i64> Sub for QuadInt<B, C> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            constant: self.constant - rhs.constant,
            first: self.first - rhs.first,
        }
    }
}

impl<const B: i64, const C: i64> SubAssign for QuadInt<B, C> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            constant: self.constant - other.constant,
            first: self.first - other.first,
        }
    }
}

impl<const B: i64, const C: i64> Mul for QuadInt<B, C> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            constant: self.constant * rhs.constant + self.first * rhs.first * C,
            first: self.first * rhs.constant
                + self.constant * rhs.first
                + self.first * rhs.first * B,
        }
    }
}

impl<const B: i64, const C: i64> MulAssign for QuadInt<B, C> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            constant: self.constant * other.constant + self.first * other.first * C,
            first: self.first * other.constant
                + self.constant * other.first
                + self.first * other.first * B,
        }
    }
}

impl<const B: i64, const C: i64> Zero for QuadInt<B, C> {
    fn zero() -> Self {
        Self {
            constant: 0,
            first: 0,
        }
    }
}

impl<const B: i64, const C: i64> Identity for QuadInt<B, C> {
    fn identity() -> Self {
        Self {
            constant: 1,
            first: 0,
        }
    }
}

impl<const B: i64, const C: i64> Neg for QuadInt<B, C> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            constant: -self.constant,
            first: -self.first,
        }
    }
}
