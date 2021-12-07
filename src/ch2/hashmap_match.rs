use std::collections::HashMap;
fn main() {
    // HashMapを生成して初期化
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);
    // キーが存在しないか確認する
    match map.get("D") {
        None => println!("Dは存在しない"),
        Some(v) => println!("D={}", v),
    }
}

