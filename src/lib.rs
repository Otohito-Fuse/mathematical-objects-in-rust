pub mod f_p;
pub mod identities;
pub mod integer;
pub mod inverse;
pub mod matrix;
pub mod modint;
pub mod polynomial;
pub mod quadratic_extension;
pub mod quadratic_field;
pub mod quadratic_integer;
pub mod rational_number;
pub mod real_number;

#[cfg(test)]
mod tests {
    use crate::f_p::is_prime;
    use crate::identities::{Identity, Zero};
    use crate::integer::Integer;
    use crate::inverse::Inverse;
    use crate::modint::ModInt;
    use crate::polynomial::Polynomial;
    use crate::quadratic_extension::QuadExt;
    use crate::quadratic_field::QuadField;
    use crate::quadratic_integer::QuadInt;
    use crate::rational_number::RationalNumber;

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
        assert!(is_prime(7));
        assert!(is_prime(101));
        assert!(!is_prime(456_789));
        assert!(is_prime(998_244_353));
        assert!(is_prime(1_000_000_007));
        assert!(!is_prime(100_000_000_000_000_000));
        // 2^64 - 1
        assert!(!is_prime(18_446_744_073_709_551_615));
        // レオンハルトオイラーが1772年に発見した素数
        assert!(is_prime(2_147_483_647));
        // トーマスクラウセンが1855年に発見した素数
        assert!(is_prime(67_280_421_310_721));
    }

    /// RationalNumberについて、異なる見た目の有理数どうしが同一視されているかの確認。
    #[test]
    fn equivalent_rational_numbers1() {
        let a = RationalNumber::new(-10, 30);
        let b = RationalNumber::new(-7, 21);
        assert_eq!(a, b);
    }

    /// RationalNumberの加算の確認。
    #[test]
    fn add_rational_numbers1() {
        let a = RationalNumber::new(3, 5);
        let b = RationalNumber::new(1, 15);
        let c = RationalNumber::new(2, 3);
        assert_eq!(a + b, c);
    }

    /// RationalNumberの減算の確認。
    #[test]
    fn subtract_rational_numbers1() {
        let a = RationalNumber::new(3, 5);
        let b = RationalNumber::new(1, 10);
        let c = RationalNumber::new(1, 2);
        assert_eq!(a - b, c);
    }

    /// RationalNumberの乗算の確認。
    #[test]
    fn multiply_rational_numbers1() {
        let a = RationalNumber::new(-3, 5);
        let b = RationalNumber::new(2, 33);
        let c = RationalNumber::new(-2, 55);
        assert_eq!(a * b, c);
    }

    /// Q\[√2\]での乗算の確認。
    #[test]
    fn q_sqrt_2_multiply_test1() {
        let a = QuadField::<0, 2>::new(RationalNumber::new(5, 8), RationalNumber::new(3, 7));
        let b = QuadField::<0, 2>::new(RationalNumber::new(7, 10), RationalNumber::new(-17, 35));
        let c = QuadField::<0, 2>::new(RationalNumber::new(83, 3920), RationalNumber::new(-1, 280));
        assert_eq!(a * b, c);
    }

    /// 多項式表示テスト
    #[test]
    fn display_polynomial_test1() {
        let v: Vec<Integer> = vec![
            Integer::new(2),
            Integer::new(3),
            Integer::new(6),
            Integer::new(0),
            Integer::new(8),
            Integer::new(0),
        ];
        let s: String = "2 + 3x + 6x^2 + 8x^4".to_string();
        let f = Polynomial::new(&v);
        assert_eq!(format!("{}", f), s);
    }

    /// 多項式表示テスト2
    #[test]
    fn display_polynomial_test2() {
        let v: Vec<Integer> = vec![
            Integer::new(0),
            Integer::new(0),
            Integer::new(3),
            Integer::new(0),
            Integer::new(1),
            Integer::new(0),
        ];
        let s: String = "3x^2 + x^4".to_string();
        let f = Polynomial::new(&v);
        assert_eq!(format!("{}", f), s);
    }

    /// 多項式の和
    #[test]
    fn add_polynomials1() {
        let v1: Vec<Integer> = vec![
            Integer::new(0),
            Integer::new(0),
            Integer::new(3),
            Integer::new(0),
            Integer::new(1),
            Integer::new(0),
        ];
        let v2: Vec<Integer> = vec![
            Integer::new(3),
            Integer::new(2),
            Integer::new(-3),
            Integer::new(5),
            Integer::new(1),
            Integer::new(1),
            Integer::new(-20),
        ];
        let v3: Vec<Integer> = vec![
            Integer::new(3),
            Integer::new(2),
            Integer::new(0),
            Integer::new(5),
            Integer::new(2),
            Integer::new(1),
            Integer::new(-20),
        ];
        let f1 = Polynomial::new(&v1);
        let f2 = Polynomial::new(&v2);
        let f3 = Polynomial::new(&v3);
        assert_eq!(f1 + f2, f3);
    }

    /// 多項式の積
    #[test]
    fn multiply_polynomials1() {
        let v1: Vec<Integer> = vec![
            Integer::new(2),
            Integer::new(0),
            Integer::new(3),
            Integer::new(0),
        ];
        let v2: Vec<Integer> = vec![
            Integer::new(3),
            Integer::new(2),
            Integer::new(-3),
            Integer::new(5),
        ];
        let v3: Vec<Integer> = vec![
            Integer::new(6),
            Integer::new(4),
            Integer::new(3),
            Integer::new(16),
            Integer::new(-9),
            Integer::new(15),
            Integer::new(0),
        ];
        let f1 = Polynomial::new(&v1);
        let f2 = Polynomial::new(&v2);
        let f3 = Polynomial::new(&v3);
        assert_eq!(f1 * f2, f3);
    }
}
