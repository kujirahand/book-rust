// Rustで人気投票の集計
// HashMapを使うのに必要な宣言 --- (*1)
use std::collections::HashMap;

// 投票データを定数として宣言 --- (*2)
const V_DATA: &str = 
    "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C";

fn main() {
    // 集計用のHashMapを生成 --- (*3)
    let mut c_map = HashMap::new();
    // HashMapを0で初期化 --- (*4)
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);
    // 投票データを集計 --- (*5)
    for w in V_DATA.split(',') {
        c_map.insert(w, c_map[w]+1);
    }
    // 集計して結果を表示 --- (*6)
    for k in ["A","B","C"] {
        println!("{}: {:>2}", k, c_map[k]);
    }
}

