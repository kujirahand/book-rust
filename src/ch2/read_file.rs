use std::env; // コマンドライン引数のため
use std::fs; // ファイルの読み込みのため

fn main() {
    // 引数をベクタとして得る --- (*1)
    let args: Vec<String> = env::args().collect();
    // ファイル名の指定があるかどうか調べる --- (*2)
    if args.len() < 2 {
        println!("入力ファイルを指定してください。");
        return;
    }
    // (0から数えて)1番目の要素を得る --- (*3)
    let filename = &args[1];
    // ファイルを読んで表示 --- (*4)
    let text = fs::read_to_string(filename).unwrap();
    println!("{}", text);
}

