use crate::identities::Zero;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// R\[x\]/(x^2 - bx - c) の元。
/// ここでRは型```T```の対象のなす環。
/// 有限体の2次拡大などを意識している。
///
/// 本当は定数B,Cを```T```型として取りたかったが、
/// ```<const B: T>```のような指定は許されていないようなので断念。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct QuadExt<T> {
    constant: T,
    first: T,
    b: T,
    c: T,
}

impl<T> QuadExt<T> {
    /// コンストラクタ。1つめの引数が定数項。2つめが1次の項。3つめと4つめが割るイデアルの生成元の1次の係数と定数項。
    pub fn new(constant: T, first: T, b: T, c: T) -> Self {
        Self {
            constant: constant,
            first: first,
            b: b,
            c: c,
        }
    }
}

/// ```println!```などで見やすく表示させるため、```Display```トレイトを実装。
/// 型```T```がそもそも```Display```トレイトを実装していることを要求。
impl<T: fmt::Display> fmt::Display for QuadExt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[({}) + ({})x]", self.constant, self.first)
    }
}

/// 足し算の実装。
/// これら演算は、bやcが異なっている場合（すなわち異なる環の元どうしでの演算を試みた場合）、
/// R\[x\] / (x^2) の元 0 を返すことにする。
impl<T: Copy + Add<Output = T> + Zero + Eq> Add for QuadExt<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if self.b != rhs.b || self.c != rhs.c {
            Self {
                constant: T::zero(),
                first: T::zero(),
                b: T::zero(),
                c: T::zero(),
            }
        } else {
            Self {
                constant: self.constant + rhs.constant,
                first: self.first + rhs.first,
                b: self.b,
                c: self.c,
            }
        }
    }
}

impl<T: Copy + Add<Output = T> + Zero + Eq> AddAssign for QuadExt<T> {
    fn add_assign(&mut self, other: Self) {
        *self = if self.b != other.b || self.c != other.c {
            Self {
                constant: T::zero(),
                first: T::zero(),
                b: T::zero(),
                c: T::zero(),
            }
        } else {
            Self {
                constant: self.constant + other.constant,
                first: self.first + other.first,
                b: self.b,
                c: self.c,
            }
        }
    }
}

impl<T: Copy + Sub<Output = T> + Zero + Eq> Sub for QuadExt<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if self.b != rhs.b || self.c != rhs.c {
            Self {
                constant: T::zero(),
                first: T::zero(),
                b: T::zero(),
                c: T::zero(),
            }
        } else {
            Self {
                constant: self.constant - rhs.constant,
                first: self.first - rhs.first,
                b: self.b,
                c: self.c,
            }
        }
    }
}

impl<T: Copy + Sub<Output = T> + Zero + Eq> SubAssign for QuadExt<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = if self.b != other.b || self.c != other.c {
            Self {
                constant: T::zero(),
                first: T::zero(),
                b: T::zero(),
                c: T::zero(),
            }
        } else {
            Self {
                constant: self.constant - other.constant,
                first: self.first - other.first,
                b: self.b,
                c: self.c,
            }
        }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Zero + Eq> Mul for QuadExt<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        if self.b != rhs.b || self.c != rhs.c {
            Self {
                constant: T::zero(),
                first: T::zero(),
                b: T::zero(),
                c: T::zero(),
            }
        } else {
            Self {
                constant: self.constant * rhs.constant + self.first * rhs.first * self.c,
                first: self.first * rhs.constant
                    + self.constant * rhs.first
                    + self.first * rhs.first * self.b,
                b: self.b,
                c: self.c,
            }
        }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Zero + Eq> MulAssign for QuadExt<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = if self.b != other.b || self.c != other.c {
            Self {
                constant: T::zero(),
                first: T::zero(),
                b: T::zero(),
                c: T::zero(),
            }
        } else {
            Self {
                constant: self.constant * other.constant + self.first * other.first * self.c,
                first: self.first * other.constant
                    + self.constant * other.first
                    + self.first * other.first * self.b,
                b: self.b,
                c: self.c,
            }
        }
    }
}
