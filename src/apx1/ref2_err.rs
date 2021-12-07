// 問題のあるプログラム
fn main() {
    // 文字列を生成
    let mut s = String::from("能ある鷹は爪を隠す");
    // 参照を得たい
    let ref1 = &mut s;
    let ref2 = &mut s; // ←ここでエラーになる
    // 画面に表示
    println!("ref1={}, ref2={}", ref1, ref2);
}

