pub mod matrix;
pub mod modint;
pub mod identities;
pub mod integer;
pub mod quadratic_integer;
pub mod inverse;

#[cfg(test)]
mod tests {
    use crate::modint::ModInt;
    use crate::integer::Integer;
    use crate::identities::{Zero,Identity};
    use crate::quadratic_integer::QuadInt;
    use crate::inverse::Inverse;
    
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
        assert_eq!("3 mod 1000000007", format!("{}",ModInt::<MOD1>::new(3)))
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
        assert_eq!("[1 + 1x]", format!("{}",QuadInt::<0, -1>::new(1, 1)))
    }
}
