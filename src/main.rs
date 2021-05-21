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
use crate::solution_set::SolutionSet;

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

fn main() {
    display_solution_set_test1();
    display_solution_set_test2();
}
