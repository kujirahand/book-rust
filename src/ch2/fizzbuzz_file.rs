use std::fs::{self, File};
use std::io::{Write, BufWriter};

fn main() {
    // 出力ファイル名を指定 --- (*1)
    let filename = "fizzbuzz_file_result.txt";
    // ファイル書き込みの有効なブロックを作る --- (*2)
    {
        // ファイルを開く --- (*3)
        let fp = File::create(filename).unwrap();
        let mut writer = BufWriter::new(fp);
        // FizzBuzzを100まで求める
        for i in 1..=100 {
            // FizzBuzzの答えを得る
            let mut line = format!("{}\n", i);
            if (i % 3 == 0) && (i % 5 == 0) {
                line = String::from("FizzBuzz\n");
            } else if i % 3 == 0 {
                line = String::from("Fizz\n");
            } else if i % 5 == 0 {
                line = String::from("Buzz\n");
            }
            // ファイルへ書き込む --- (*4)
            let bytes = line.as_bytes();
            writer.write(bytes).unwrap();
        }
    } // ←ここで自動的にファイルは閉じられる --- (*5)
    
    // 保存したファイルの内容を読み込んで表示 --- (*6)
    let s = fs::read_to_string(filename).unwrap();
    println!("{}", s);
}

