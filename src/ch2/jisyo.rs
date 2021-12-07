use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    // 辞書ファイルの指定 --- (*1)
    let dicfile = "ejdict-hand-utf8.txt";
    
    // コマンドライン引数をベクタに入れる --- (*2)
    let args: Vec<String> = std::env::args().collect();
    
    // 引数のチェック
    if args.len() < 2 {
        println!("[USAGE] jisyo word");
        return;
    }
    // 指定された単語 --- (*3)
    let word = &args[1];
    
    // ファイルを開く --- (*4)
    let fp = File::open(dicfile).unwrap();
    // BufReaderで一行ずつ読む --- (*5)
    let reader = BufReader::new(fp);
    for line in reader.lines() {
        // 実際に一行取り出す --- (*6)
        let line = line.unwrap();
        // 行に単語が含まれるか検索 --- (*7)
        if line.find(word) == None { continue; }
        println!("{}", line);
    }
}

