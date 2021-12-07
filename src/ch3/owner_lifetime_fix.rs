// メッセージを生成する関数
fn gen_message() -> String {
    let msg = String::from("過ちを見過ごす人は美しい");
    return msg; // 所有権が関数の戻り値に移動 --- (*1)
}
fn main() {
    let m = gen_message(); // 所有権はmに移動 --- (*2)
    println!("{}", m); // ok
}

