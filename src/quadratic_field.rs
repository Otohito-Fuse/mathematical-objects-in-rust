use crate::identities::{Identity, Zero};
use crate::rational_number::RationalNumber;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Q\[x\]/(x^2 - Bx - C) の元（B, Cは整数ｓ）
///
/// B = 0, C = 2 のときは Q\[sqrt(2)\] などs
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct QuadField<const B: i64, const C: i64> {
    constant: RationalNumber,
    first: RationalNumber,
}

impl<const B: i64, const C: i64> QuadField<B, C> {
    /// コンストラクタ。1つめの引数が定数項。2つめが1次の項。
    pub fn new(constant: RationalNumber, first: RationalNumber) -> Self {
        Self {
            constant: constant,
            first: first,
        }
    }
}

/// ```println!```などで見やすく表示させるため、```Display```トレイトを実装。
impl<const B: i64, const C: i64> fmt::Display for QuadField<B, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[({}) + ({})x]", self.constant, self.first)
    }
}

impl<const B: i64, const C: i64> Add for QuadField<B, C> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            constant: self.constant + rhs.constant,
            first: self.first + rhs.first,
        }
    }
}

impl<const B: i64, const C: i64> AddAssign for QuadField<B, C> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            constant: self.constant + other.constant,
            first: self.first + other.first,
        }
    }
}

impl<const B: i64, const C: i64> Sub for QuadField<B, C> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            constant: self.constant - rhs.constant,
            first: self.first - rhs.first,
        }
    }
}

impl<const B: i64, const C: i64> SubAssign for QuadField<B, C> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            constant: self.constant - other.constant,
            first: self.first - other.first,
        }
    }
}

impl<const B: i64, const C: i64> Mul for QuadField<B, C> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            constant: self.constant * rhs.constant
                + self.first * rhs.first * RationalNumber::new(C, 1),
            first: self.first * rhs.constant
                + self.constant * rhs.first
                + self.first * rhs.first * RationalNumber::new(B, 1),
        }
    }
}

impl<const B: i64, const C: i64> MulAssign for QuadField<B, C> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            constant: self.constant * other.constant
                + self.first * other.first * RationalNumber::new(C, 1),
            first: self.first * other.constant
                + self.constant * other.first
                + self.first * other.first * RationalNumber::new(B, 1),
        }
    }
}

impl<const B: i64, const C: i64> Zero for QuadField<B, C> {
    fn zero() -> Self {
        Self {
            constant: RationalNumber::zero(),
            first: RationalNumber::zero(),
        }
    }
}

impl<const B: i64, const C: i64> Identity for QuadField<B, C> {
    fn identity() -> Self {
        Self {
            constant: RationalNumber::identity(),
            first: RationalNumber::zero(),
        }
    }
}

impl<const B: i64, const C: i64> Neg for QuadField<B, C> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            constant: -self.constant,
            first: -self.first,
        }
    }
}
