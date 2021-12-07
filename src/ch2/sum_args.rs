fn main() {
    // コマンドライン引数を得る --- (*1)
    let args = std::env::args();
    let mut total = 0.0;
    // コマンドライン引数を順に加算 --- (*2)
    for (i, s) in args.enumerate() {
        // 0番目はコマンド自身なので飛ばす --- (*3)
        if i == 0 { continue; }
        // コマンドライン引数を数値に変換 --- (*4)
        let num: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };
        total += num;
    }
    // 結果を表示 --- (*5)
    println!("{}", total);
}

