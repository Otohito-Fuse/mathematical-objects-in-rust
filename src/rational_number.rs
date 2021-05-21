use crate::identities::{Identity, Zero};
use crate::inverse::Inverse;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// 有理数（分数）を表現するための```RationalNumber```構造体の定義。
///
/// 常に既約分数、分母は正とすることで一意性を担保し、```Eq```トレイトも自動的に実装させる。
/// ただし、0は0/1とする。また、分母が0になったものは便宜上1/0とする。
///
/// 分子は```i64```型、分母は```u64```型とする。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct RationalNumber {
    numerator: i64,
    denominator: u64,
}

impl RationalNumber {
    pub fn new(n: i64, d: u64) -> Self {
        if d == 0 {
            Self {
                numerator: 1,
                denominator: 0,
            }
        } else if n == 0 {
            Self {
                numerator: 0,
                denominator: 1,
            }
        } else {
            let m = if n >= 0 { n as u64 } else { -n as u64 };
            let gcd = num::Integer::gcd(&m, &d);
            Self {
                numerator: n / (gcd as i64),
                denominator: d / gcd,
            }
        }
    }

    pub fn to_int(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl fmt::Display for RationalNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) / ({})", self.numerator, self.denominator)
    }
}

impl Add for RationalNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        RationalNumber::new(
            self.numerator * rhs.denominator as i64 + self.denominator as i64 * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl AddAssign for RationalNumber {
    fn add_assign(&mut self, other: Self) {
        *self = RationalNumber::new(
            self.numerator * other.denominator as i64 + self.denominator as i64 * other.numerator,
            self.denominator * other.denominator,
        );
    }
}

impl Sub for RationalNumber {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        RationalNumber::new(
            self.numerator * rhs.denominator as i64 - self.denominator as i64 * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl SubAssign for RationalNumber {
    fn sub_assign(&mut self, other: Self) {
        *self = RationalNumber::new(
            self.numerator * other.denominator as i64 - self.denominator as i64 * other.numerator,
            self.denominator * other.denominator,
        );
    }
}

impl Mul for RationalNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        RationalNumber::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl MulAssign for RationalNumber {
    fn mul_assign(&mut self, other: Self) {
        *self = RationalNumber::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        );
    }
}

impl Zero for RationalNumber {
    fn zero() -> Self {
        RationalNumber::new(0, 1)
    }
}

impl Identity for RationalNumber {
    fn identity() -> Self {
        RationalNumber::new(1, 1)
    }
}

/// 逆元の実装
impl Inverse for RationalNumber {
    /// 整数の逆元。1,-1のみ逆元を持つ。
    fn inverse(self) -> Option<RationalNumber> {
        if self.numerator == 0 {
            None
        } else if self.numerator > 0 {
            Some(RationalNumber::new(
                self.denominator as i64,
                self.numerator as u64,
            ))
        } else {
            Some(RationalNumber::new(
                -(self.denominator as i64),
                (-self.numerator) as u64,
            ))
        }
    }
}
