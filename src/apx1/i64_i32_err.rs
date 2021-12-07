fn main() {
    // i32型の変数を定義
    let n: i32 = 100;
    // i64型にi32型の値を代入
    let m: i64 = n; // ←ここでエラー
    println!("{},{}", n, m);
}

