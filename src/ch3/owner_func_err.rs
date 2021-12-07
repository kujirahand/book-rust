// 問題のあるプログラム
fn main() {
    let g1 = String::from("過ちを見過ごす人は美しい");
    show_message(g1); // 所有権が移動する
    println!("{}", g1); // g1は既に使えない
}

fn show_message(message: String) {
    println!("{}", message);
}

