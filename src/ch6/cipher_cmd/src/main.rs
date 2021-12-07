mod cipher_str;
use std::env;

fn main() {
    // コマンドライン引数を確認 --- (*1)
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        show_usage(); return;
    }
    // コマンドライン引数から値を得る
    let method = String::from(args[1].trim());
    let password = String::from(args[2].trim());
    let data = String::from(args[3].trim());
    // 暗号化・復号化 --- (*2)
    let result = match &method[..] {
        "enc" => cipher_str::encrypt(&password, &data),
        "dec" => cipher_str::decrypt(&password, &data),
        _ => { show_usage(); return; },
    };
    println!("{}", result);
}

fn show_usage() {
    println!("[USAGE] cipher_cmd (enc|dec) password data");
}
