# mathematical-objects-in-rust
Rust の構造体定義やトレイトの扱いの練習のために、いくつかの数学的概念を Rust で表現してみたもの。

## Rust のバージョンについて
"const generics" （```struct A<const N: usize>```みたいなやつ）を使っているので、Rust 1.50 以降である必要がある。

## それぞれのモジュールの意味
### modint
```ModInt<N>```型の対象は Z / NZ の元と同一視される。演算子 ```+```, ```-```, ```*``` をオーバーロードしてある。
逆元を求める```Inverse```トレイトも実装済み。

### identities
零元や単位元に相当するものが欲しいときのために ```Zero```, ```Identity``` の2つのトレイトを定義

### integer
```Zero```, ```Identity``` などの自作トレイトを実装するために ```i64``` 型のフィールドを1つだけ持つ構造体を別に定義。
各種演算子をオーバーロード。

### matrix
足し算、掛け算、零元、単位元に相当する各トレイトを実装している型```T```を持つ対象を成分にもつ行列。
行列の掛け算や繰り返し二乗法によるべき乗などの関数を用意。

### quadratic integer
Z\[X\] / (X^2 - BX - C) の元。足し算、引き算、掛け算を実装。

### inverse
逆元を返す関数```inverse()```を持っていることを要求するトレイト。
```inverse()```は、可逆元のときは```Some(逆元)```、そうでないときは```None```を返す。

### quadratic extension
quadratic integer と同じことを任意の環についてやる。

すなわち、R[x]/(x^2 - bx - c) の元。
ここでRは型```T```の対象のなす環（```T```は```Add```,```Sub```,```Mul```や```Eq```,```Copy```を実装している必要がある）。
有限体の2次拡大などを意識している。

本当は定数b,cを```T```型として取りたかったが、
```<const B: T>```のような指定は許されていないようなので断念。
定数b,cも構造体内部のフィールドとして保持し、
異なる環の元どうしの演算を試みたときは一環して 0 in R[x]/(x^2) を返すようにした。

その結果```Zero```や```Identity```は実装できなかった。

### F_p
有限体 F_p (pは素数) 周りの概念。
今は関数```is_prime: f64 -> bool```のみ定義。

### real number
```Zero```, ```Identity``` などの自作トレイトを実装するために ```f64``` 型のフィールドを1つだけ持つ構造体を別に定義。
各種演算子をオーバーロード。

```Inverse```も実装したかったが、```0```かどうかの判定が難しいのでとりあえず保留。