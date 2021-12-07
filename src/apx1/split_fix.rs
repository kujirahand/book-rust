fn main() {
    let target = "aaa,bbb,ccc";
    // for文を使ってVec<String>に変換
    let mut lines = vec![];
    for line in target.split(",") {
        lines.push(line.to_string());
    }
    println!("{:?}", lines);
    // mapを使ってVec<String>に変換
    let lines:Vec<String> = 
        target.split(",").map(|s| s.to_string()).collect();
    println!("{:?}", lines);
}

