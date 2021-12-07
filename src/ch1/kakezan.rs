// 掛け算をするだけの関数を定義
fn kakezan(a: i64, b: i64) -> i64 {
    a * b
}

fn main() {
    // 関数を呼び出す
    let ex1 = kakezan(3, 5);
    println!("3*5={}", ex1);
    let ex2 = kakezan(8, 4);
    println!("8*4={}", ex2);
}
