use std::{env, path};

fn main() {
    // コマンドライン引数を確認 --- (*1)
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("findfile (path) (keyword)");
        return;
    }
    // コマンドライン引数を得る --- (*2)
    let target_dir = &args[1];
    let keyword = &args[2];
    // PathBufに変換 --- (*3)
    let target = path::PathBuf::from(target_dir);    
    findfile(&target, keyword);
}

// 再帰的にファイルを検索する関数 --- (*4)
fn findfile(target: &path::PathBuf, keyword: &str) {
    // ファイル一覧を取得 --- (*5)
    let files = target.read_dir().expect("存在しないパス");
    for dir_entry in files {
        // PathBufを得る --- (*6)
        let path = dir_entry.unwrap().path();
        // ディレクトリならば再帰的に検索 --- (*7)
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }
        // ファイル名を文字列に変換 --- (*8)
        let fname = path.file_name().unwrap()
            .to_string_lossy();
        // キーワードを含むかチェック --- (*9)
        if None == fname.find(keyword) { continue; }
        // キーワードを含むパスを表示
        println!("{}", path.to_string_lossy());
    }
}

