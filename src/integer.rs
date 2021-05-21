use crate::identities::{Identity, Zero};
use crate::inverse::Inverse;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// 加減乗除など以外にもトレイトを実装するため、```i64```型と実質的には同等な```Integer```構造体を新たに定義。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Integer {
    value: i64,
}

impl Integer {
    pub fn new(n: i64) -> Self {
        Integer { value: n }
    }

    pub fn to_int(&self) -> i64 {
        self.value
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value >= 0 {
            write!(f, "{}", self.value)
        } else {
            write!(f, "({})", self.value)
        }
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

/// 逆元の実装
impl Inverse for Integer {
    /// 整数の逆元。1,-1のみ逆元を持つ。
    fn inverse(self) -> Option<Integer> {
        match self {
            Integer { value: 1 } => Some(Integer { value: 1 }),
            Integer { value: -1 } => Some(Integer { value: -1 }),
            _ => None,
        }
    }
}
