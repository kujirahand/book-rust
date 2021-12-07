fn main() {
    // 配列に文字列を代入
    let array = [
        "Apple".to_string(), 
        "Banana".to_string(), 
        "Mango".to_string(), 
        "Tomato".to_string()
    ];
    // 配列を繰り返し画面に表示する
    for a in array.iter() { // ←修正
        println!("{}", a);
    }
    println!("len={}", array.len()); // ←ok
}

