use std::fs;
fn main() {
    // ファイル一覧を取得 --- (*1)
    let files = fs::read_dir(".").expect("不正なパス");
    for ent in files {
        // エントリを得る --- (*2)
        let entry = ent.unwrap();
        // PathBufを得る --- (*3)
        let path = entry.path();
        // ファイル名を表示 --- (*4)
        let fname = path.to_str().unwrap_or("不正なファイル名");
        println!("{}", fname);
    }
}

