use crate::identities::{Identity, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// 加減乗除など以外にもトレイトを実装するため、```f64```型と実質的には同等な```RealNumber```構造体を新たに定義。
#[derive(Debug, Clone, Copy)]
pub struct RealNumber {
    value: f64,
}

impl RealNumber {
    pub fn new(r: f64) -> Self {
        RealNumber { value: r }
    }

    pub fn to_f64(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for RealNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for RealNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for RealNumber {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            value: self.value + other.value,
        };
    }
}

impl Sub for RealNumber {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for RealNumber {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            value: self.value - other.value,
        };
    }
}

impl Mul for RealNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for RealNumber {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            value: self.value * other.value,
        };
    }
}

impl Neg for RealNumber {
    type Output = Self;
    fn neg(self) -> Self {
        Self { value: -self.value }
    }
}

impl Zero for RealNumber {
    fn zero() -> Self {
        RealNumber::new(0f64)
    }
}

impl Identity for RealNumber {
    fn identity() -> Self {
        RealNumber::new(1f64)
    }
}
