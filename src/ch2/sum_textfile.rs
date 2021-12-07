use std::{env, fs};

fn main() {
    // コマンドライン引数を得る --- (*1)
    let args = env::args();
    let mut total:f64 = 0.0;
    // 全てのコマンドライン引数を処理 --- (*2)
    for (i, fname) in args.enumerate() {
        if i == 0 { continue; }
        // テキストファイルを読む --- (*3)
        let text = fs::read_to_string(fname).unwrap();
        // 一行ごとに区切る --- (*4)
        let lines = text.split('\n');
        // 繰り返し加算 --- (*5)
        for line in lines {
            // 数値に変換 --- (*6)
            let n:f64 = match line.parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };
            total += n;
        }
    }
    // 結果を表示 --- (*7)
    println!("{}", total);
}


