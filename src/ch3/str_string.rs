fn main() {
    // 文字列リテラルは &str 型 --- (*1)
    let ss: &str = "気前よく与えるなら豊かになる";
    // &str から String への変換 --- (*2)
    let so1: String = String::from(ss);
    let so2: String = ss.to_string();
    // String から str への変換 --- (*3)
    let ss2: &str = &so1;
    let ss3: &str = so1.as_str();
    // 画面に表示
    println!("{}\n{}\n{}\n{}", so1, so2, ss2, ss3);
    // 参照型のポインタアドレスを表示 --- (*4)
    println!("{:p}\n{:p}", ss2, ss3);
}

