fn main() {
    // コマンドライン引数をVec<String>で得る
    let args:Vec<String> = std::env::args().collect();
    println!("{:?}", args);
}

