pub mod f_p;
pub mod identities;
pub mod integer;
pub mod inverse;
pub mod matrix;
pub mod modint;
pub mod quadratic_extension;
pub mod quadratic_integer;

#[cfg(test)]
mod tests {
    use crate::f_p::is_prime;
    use crate::identities::{Identity, Zero};
    use crate::integer::Integer;
    use crate::inverse::Inverse;
    use crate::modint::ModInt;
    use crate::quadratic_extension::QuadExt;
    use crate::quadratic_integer::QuadInt;

    const MOD1: u64 = 1_000_000_007;
    const MOD2: u64 = 998_244_353;

    /// Integer型がZero, Identityの2つのトレイトを想定通りに実装できているかどうか
    #[test]
    fn integer_treatment1() {
        assert_eq!(Integer::new(0), Integer::zero());
        assert_eq!(Integer::new(1), Integer::identity());
    }

    /// Integer型について、3は逆元を持たないこと、-1は自身を逆元としてもっていることなどを確認
    #[test]
    fn inverse_of_integer1() {
        let n = Integer::new(3);
        let m = Integer::new(-1);
        assert_eq!(n.inverse(), None);
        assert_eq!(m.inverse().unwrap(), m);
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
        assert_eq!("3 mod 1000000007", format!("{}", ModInt::<MOD1>::new(3)))
    }

    /// ModInt型がInverseトレイトを想定通りに実装できているかどうか。
    /// mod 13 で互いに逆元の関係にある5と8について確認。
    #[test]
    fn inverse_of_modint1() {
        let n = ModInt::<13>::new(5);
        let m = ModInt::<13>::new(8);
        assert_eq!(n.inverse().unwrap(), m);
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
        assert_eq!("[1 + 1x]", format!("{}", QuadInt::<0, -1>::new(1, 1)))
    }

    /// QuadExt型の足し算と掛け算の確認。
    /// F_11[X] / (X^2 + 1) で確かめる。
    #[test]
    fn quadratic_extension_treatment1() {
        let minus_one = ModInt::<11>::new(10);
        let zero = ModInt::<11>::zero();
        // 1 + i in F_11(sqrt(-1))
        let x =
            QuadExt::<ModInt<11>>::new(ModInt::<11>::new(1), ModInt::<11>::new(1), zero, minus_one);
        // 3 + 6i in F_11(sqrt(-1))
        let y =
            QuadExt::<ModInt<11>>::new(ModInt::<11>::new(3), ModInt::<11>::new(6), zero, minus_one);
        // 4 + 7i in F_11(sqrt(-1))
        let z =
            QuadExt::<ModInt<11>>::new(ModInt::<11>::new(4), ModInt::<11>::new(7), zero, minus_one);
        // 8 + 9i in F_11(sqrt(-1))
        let w =
            QuadExt::<ModInt<11>>::new(ModInt::<11>::new(8), ModInt::<11>::new(9), zero, minus_one);
        assert_eq!(x + y, z);
        assert_eq!(x * y, w);
    }

    /// is_prime関数の確認。
    #[test]
    fn is_prime_check1() {
        assert!(is_prime(7).unwrap());
        assert!(is_prime(101).unwrap());
        assert!(!is_prime(456789).unwrap());
        assert!(is_prime(998244353).unwrap());
        assert!(is_prime(1000000007).unwrap());
        assert!(!is_prime(10000000000000).unwrap());
        assert_eq!(is_prime(100000000000001), None);
    }
}
