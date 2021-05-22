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
pub mod solution_set;

use crate::integer::Integer;
use crate::modint::ModInt;
use crate::polynomial::Polynomial;
use crate::solution_set::SolutionSet;

use crate::f_p::{solve_equation, MOD};

use std::collections::HashSet;

/// 解集合に実装したDisplayトレイトの振る舞いを確認するその1
fn display_solution_set_test1() {
    let mut s = SolutionSet::new(HashSet::<Integer>::new());
    s.insert(Integer::new(0));
    s.insert(Integer::new(2));
    s.insert(Integer::new(-15));
    s.insert(Integer::new(6));
    s.insert(Integer::new(8));
    println!("{}", s);
}

/// 解集合に実装したDisplayトレイトの振る舞いを確認するその2
fn display_solution_set_test2() {
    let s = SolutionSet::new(HashSet::<Integer>::new());
    println!("{}", s);
}

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
    assert_eq!(f1.clone() * f2.clone(), f3);
    println!("({}) * ({}) = {}", f1, f2, f3);
}

fn derivative_of_polynomial1() {
    let v1: Vec<Integer> = vec![
        Integer::new(2),
        Integer::new(2),
        Integer::new(3),
        Integer::new(-1),
    ];
    let v2: Vec<Integer> = vec![
        Integer::new(2),
        Integer::new(6),
        Integer::new(-3),
        Integer::new(0),
    ];
    let f1 = Polynomial::new(&v1);
    let f2 = Polynomial::new(&v2);
    println!("({})\' = {}", f1, f2);
}

fn derivative_of_polynomial2() {
    let v1: Vec<ModInt<7>> = vec![
        ModInt::<7>::new(1),
        ModInt::<7>::new(0),
        ModInt::<7>::new(0),
        ModInt::<7>::new(0),
        ModInt::<7>::new(0),
        ModInt::<7>::new(0),
        ModInt::<7>::new(0),
        ModInt::<7>::new(1),
    ];
    let v2: Vec<ModInt<7>> = vec![ModInt::<7>::new(0)];
    let f1 = Polynomial::new(&v1);
    let f2 = Polynomial::new(&v2);
    println!("({})\' = {} in F_p[x]", f1, f2);
}

fn main() {
    display_solution_set_test1();
    display_solution_set_test2();

    multiply_polynomials1();

    derivative_of_polynomial1();
    derivative_of_polynomial2();

    // F_103での方程式 x^3+3x^2+95x+4 = 0 の解を全探索
    let v: Vec<ModInt<MOD>> = vec![
        ModInt::<MOD>::new(4),
        ModInt::<MOD>::new(95),
        ModInt::<MOD>::new(3),
        ModInt::<MOD>::new(1),
    ];
    let f1 = Polynomial::new(&v);
    let s = solve_equation(f1.clone());
    println!(
        "The solution set of the equation {} = 0 in F_{} is {}",
        f1, MOD, s
    );
}
