// (注意) エラーのあるプログラム
use std::collections::HashMap;
fn main() {
    // HashMapを生成して初期化
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);
    // 存在しないキーを取得しようとした!!
    let d = map["D"];
    println!("{}", d);
}

