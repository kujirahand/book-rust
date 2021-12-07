// 問題のあるプログラム
fn main() {
    let s = "気前よく与えて豊かになる人がいる".to_string();
    echo(s); // ← 所有権が移動してしまう
    println!("{}", s);
}
// println!を模して作った関数
fn echo(s: String) {
    println!("{}", s);
}

