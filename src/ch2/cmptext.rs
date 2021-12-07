// ファイル操作のライブラリを読む --- (*1)
use std::fs;
fn main() {
    // ファイル名を指定 --- (*2)
    let afile = "./fizzbuzz_python.txt";
    let bfile = "./fizzbuzz_rust.txt";
    
    // ファイルを文字列として読む --- (*3)
    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();
    
    // 念のためトリム
    let astr = astr.trim();
    let bstr = bstr.trim();

    // 比較 --- (*4)
    if astr == bstr {
        println!("ok");
    } else {
        println!("ng");
    }
}

