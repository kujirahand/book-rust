// 問題があるプログラム
fn main() {
    // 配列に文字列を代入
    let array = [
        "Apple".to_string(), 
        "Banana".to_string(), 
        "Mango".to_string(), 
        "Tomato".to_string()
    ];
    // 配列を繰り返し画面に表示したいが...
    for a in array { // ここで所有権が移動する
        println!("{}", a);
    }
    println!("len={}", array.len()); // ←エラー
}

