fn main() {
    let mut height;
    // 繰り返しを記述する --- (*1)
    loop {
        println!("身長(cm)は?");
        height = input_f(0.0); // 数値入力
        if height > 0.0 { break; }
        println!("正しい数値を入力してください。");
    }
    // 標準体重を計算して表示 --- (*2)
    let weight = 22.0 * (height / 100.0).powf(2.0);
    println!("標準体重は{:.1}kgです。", weight);
}

// 標準入力から文字列を得る --- (*3)
fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("入力エラー");
    s.trim_end().to_string()
}

// 標準入力から実数を入力(失敗したらdefを返す) --- (*4)
fn input_f(def: f64) -> f64 {
    let s = input_str();
    match s.trim().parse() {
        Ok(v) => v,
        Err(_) => def,
    }
}

