// ジェネリクスを使ってaddを定義
fn add <T: std::ops::Add<Output=T>> (a:T, b:T) -> T {
    a + b
}
// 関数を使ってみる
fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
    println!("{}", add::<i32>(10, 25)); // 型を明示するとき
    println!("{}", add('a', 'a'));
}

