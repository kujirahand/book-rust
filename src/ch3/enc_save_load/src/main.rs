use std::fs;
use std::fs::File;
use std::io::Write;
use encoding_rs;

fn main() {
    // 保存ファイル名を指定
    let filename = "test-sjis.txt";
    // Shift_JISで保存 --- (*1)
    save_sjis(filename, "こっそり食べる物はおいしい。");
    // Shift_JISを読み込み --- (*2)
    let s = load_sjis(filename);
    println!("{}", s);
}

fn save_sjis(filename: &str, text: &str) {
    // Shift_JISにエンコード -- (*3)
    let (enc, _, _) = encoding_rs::SHIFT_JIS.encode(text);
    let buf = enc.into_owned();
    // ファイルにバイナリを保存 --- (*4)
    let mut file = File::create(filename).expect("作成");
    file.write(&buf[..]).expect("書込");
}

fn load_sjis(filename: &str) -> String {
    // ファイルからバイト列を一気に読み込む --- (*5)
    let buf = fs::read(filename).expect("読込");
    // Shift_JISでデコード --- (*6)
    let (dec, _, _) = encoding_rs::SHIFT_JIS.decode(&buf);
    return dec.into_owned();
}

