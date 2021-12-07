// 問題のあるプログラム
fn int_to_str(value: i64) -> &str {
    let s = format!("{}", value);
    return &s;
}
fn main() {
    let s = int_to_str(256);
    println!("{}", s);
}

