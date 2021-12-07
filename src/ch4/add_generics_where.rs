fn add <T> (a:T, b:T) -> T
    where T: std::ops::Add<Output=T>
{
    a + b
}
// 関数を使ってみる
fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
}

