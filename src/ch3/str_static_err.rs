// &'static strのみ指定できる関数 --- (*1)
fn echo(s: &'static str) {
    println!("{}", s);
}
fn main() {
    // 文字列リテラル(&'static str)を指定 --- (*2)
    echo("愚かな人でも黙っていると");
    echo("賢いと見られる");

    // 以下のコメント部分は失敗する --- (*3)
    let s = String::from("テスト");
    echo(&s);
}

