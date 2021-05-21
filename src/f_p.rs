// 有限体 F_p (pは素数) まわりの関数の定義

/// 素数判定
///
/// nがデカすぎるときは計算を拒否させたいのでOption型を返す。
/// 具体的には n >= 2^40 ぐらいで拒否する（偶数かどうかぐらいは判定する）。
pub fn is_prime(n: u64) -> Option<bool> {
    if n == 2 {
        return Some(true);
    }
    if n % 2 == 0 {
        return Some(false);
    }
    if n >= 1 << 40 {
        return None;
    }
    if n == 0 || n == 1 {
        return Some(false);
    }
    for i in 0..n {
        if n % (3 + 2 * i) == 0 {
            return Some(false);
        }
        if (3 + 2 * i) * (3 + 2 * i) >= n {
            break;
        }
    }
    Some(true)
}
