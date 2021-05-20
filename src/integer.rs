use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use crate::units::{Zero,Identity};
use std::fmt;

/// 加減乗除など以外にもトレイトを実装するため、i64型と実質的には同等なInteger構造体を新たに定義。
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Integer {
    value: i64,
}

impl Integer {
    pub fn new(n: i64) -> Self {
        Integer {
            value: n,
        }
    }

    pub fn to_int(&self) -> i64 {
        self.value
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for Integer {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Integer {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            value: self.value + other.value,
        };
    }
}

impl Sub for Integer {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Integer {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            value: self.value - other.value,
        };
    }
}

impl Mul for Integer {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Integer {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            value: self.value * other.value,
        };
    }
}

impl Zero for Integer {
    fn zero() -> Self {
        Integer::new(0)
    }
}

impl Identity for Integer {
    fn identity() -> Self {
        Integer::new(1)
    }
}