pub mod matrix;
pub mod modint;
pub mod units;
pub mod integer;
pub mod quadratic_integer;

/* use modint::ModInt;
use integer::Integer;
use crate::units::{Zero,Identity}; */

#[cfg(test)]
mod tests {
    use crate::modint::ModInt;
    use crate::integer::Integer;
    use crate::units::{Zero,Identity};
    use crate::quadratic_integer::QuadInt;
    
    const MOD1: u64 = 1_000_000_007;
    const MOD2: u64 = 998_244_353;

    /// Integer型がZero, Identityの2つのトレイトを想定通りに実装できているかどうか
    #[test]
    fn integer_treatment1() {
        assert_eq!(Integer::new(0), Integer::zero());
        assert_eq!(Integer::new(1), Integer::identity());
    }

    /// ModInt型がZero, Identityの2つのトレイトを想定通りに実装できているかどうか
    #[test]
    fn modint_treatment1() {
        assert_eq!(ModInt::<MOD1>::new(0), ModInt::<MOD1>::zero());
        assert_eq!(ModInt::<MOD1>::new(1), ModInt::<MOD1>::identity());
        assert_eq!(ModInt::<MOD2>::new(0), ModInt::<MOD2>::zero());
        assert_eq!(ModInt::<MOD2>::new(1), ModInt::<MOD2>::identity());
    }

    /// ModInt型がDisplayトレイトを想定通りに実装できているかどうか
    #[test]
    fn modint_print1() {
        assert_eq!("3 mod 1000000007", format!("{}",ModInt::<MOD1>::new(3)))
    }

    /// QuadInt型の足し算と掛け算の確認
    #[test]
    fn quadratic_integer_treatment1() {
        // 1 + i in Z[i]
        let x = QuadInt::<0, -1>::new(1, 1);
        // 3 - 5i in Z[i]
        let y = QuadInt::<0, -1>::new(3, -5);
        // 4 - 4i in Z[i]
        let z = QuadInt::<0, -1>::new(4, -4);
        // 8 - 2i in Z[i]
        let w = QuadInt::<0, -1>::new(8, -2);
        assert_eq!(x + y, z);
        assert_eq!(x * y, w);
    }

    /// QuadInt型がDisplayトレイトを想定通りに実装できているかどうか
    #[test]
    fn quadratic_integer_print1() {
        assert_eq!("[1 + 1x]", format!("{}",QuadInt::<0, -1>::new(1, 1)))
    }
}
