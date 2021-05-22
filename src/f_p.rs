use super::identities::Zero;
use super::modint::ModInt;
use super::polynomial::Polynomial;
use super::solution_set::SolutionSet;

use std::collections::HashSet;

// 有限体 F_p (pは素数) まわりのもろもろ

/// 素数判定
pub fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    if n == 0 || n == 1 {
        return false;
    }
    for i in 0..n {
        if n % (3 + 2 * i) == 0 {
            return false;
        }
        if (3 + 2 * i) * (3 + 2 * i) >= n {
            break;
        }
    }
    true
}

pub const MOD: u64 = 103; // テキトー

/// 方程式の解を全探索
pub fn solve_equation(f: Polynomial<ModInt<MOD>>) -> SolutionSet<ModInt<MOD>> {
    let mut s: HashSet<ModInt<MOD>> = HashSet::new();
    for i in 0..MOD {
        if Polynomial::evaluate(&f, ModInt::<MOD>::new(i)) == ModInt::<MOD>::zero() {
            s.insert(ModInt::<MOD>::new(i));
        }
    }
    SolutionSet::new(s)
}
