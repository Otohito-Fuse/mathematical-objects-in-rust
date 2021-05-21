use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use crate::identities::{Zero,Identity};
use crate::inverse::Inverse;
use std::fmt;

/// MODで割った余り。\mathbb{Z} / MOD \mathbb{Z}の元。
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ModInt<const MOD: u64> {
    representative: u64,
}

impl<const MOD: u64> ModInt<MOD> {
    /// コンストラクタ。代表元が一意になるよう0以上MOD未満の整数として格納。
    pub fn new(n: u64) -> Self {
        ModInt {
            representative: n % MOD,
        }
    }

    /// u64型にする
    pub fn to_int(&self) -> u64 {
        self.representative
    }

    /// 繰り返し二乗法によるべき乗の計算
    pub fn modpow(&self, n: u64) -> Self {
        let mut res = 1;
        let mut a = self.representative;
        let mut m = n;
        loop{
            if m == 0 {
                break;
            }
            if m % 2 == 1 {
                res = (res * a) % MOD;
            }
            a = (a * a) % MOD;
            m = m / 2;
        }
        ModInt {
            representative: res,
        }
    }
}

/// println!などで見やすく表示させるため、Displayトレイトを実装。
impl<const MOD: u64> fmt::Display for ModInt<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} mod {}", self.representative, MOD)
    }
}

impl<const MOD: u64> Add for ModInt<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        ModInt {
            representative: (self.representative + rhs.representative) % MOD,
        }
    }
}

impl<const MOD: u64> AddAssign for ModInt<MOD> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            representative: (self.representative + other.representative) % MOD,
        };
    }
}

impl<const MOD: u64> Sub for ModInt<MOD> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        ModInt {
            representative: (self.representative + MOD - rhs.representative) % MOD,
        }
    }
}

impl<const MOD: u64> SubAssign for ModInt<MOD> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            representative: (self.representative + MOD - other.representative) % MOD,
        };
    }
}

impl<const MOD: u64> Mul for ModInt<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        ModInt {
            representative: (self.representative * rhs.representative) % MOD,
        }
    }
}

impl<const MOD: u64> MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            representative: (self.representative * other.representative) % MOD,
        };
    }
}

impl<const MOD: u64> Zero for ModInt<MOD> {
    fn zero() -> Self {
        ModInt::new(0)
    }
}

impl<const MOD: u64> Identity for ModInt<MOD> {
    fn identity() -> Self {
        ModInt::new(1)
    }
}

impl<const MOD: u64> Inverse for ModInt<MOD> {
    fn inverse(self) -> Option<ModInt<MOD>> {
        let n = self.to_int();
        if num::Integer::gcd(&n,&MOD) != 1 {
            None
        } else {
            let ret = self.modpow(MOD - 2);
            Some(ret)
        }
    }
}