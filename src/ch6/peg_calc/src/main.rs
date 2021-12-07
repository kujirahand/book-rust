peg::parser!( grammar calc() for str {
    
    // ルートとなる規則 --- (*1)
    pub rule eval() -> i64 
        = expr()
    
    // 足し算と引き算を行うルールを追加 --- (*2)
    rule expr() -> i64
        = l:term() "+" r:expr()   { l + r }
        / l:term() "-" r:expr()   { l - r }
        / term()

    // 掛け算と割り算を行うルールを追加 --- (*3)
    rule term() -> i64
        = l:value() "*" r:term() { l * r }
        / l:value() "/" r:term() { l / r }
        / v:value()

    // 値を読むルールを追加 --- (*4)
    rule value() -> i64
        = number()                  // 数値
        / "(" v:expr() ")" { v }   // (計算式)
        
    // 数値を取得するルールを指定 --- (*5)
    rule number() -> i64
        = n:$(['0'..='9']+) 
        { n.parse().unwrap() }
});

fn main() {
    // 計算を試してみる
    println!("{}", calc::eval("1+2*3").unwrap());
    println!("{}", calc::eval("(1+2)*3").unwrap());
    println!("{}", calc::eval("100/2-1").unwrap());    
}

