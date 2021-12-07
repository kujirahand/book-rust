// 以下に文法規則を記述 --- (*1)
peg::parser!( grammar calc() for str {
    // ルートとなる規則を追加 --- (*2)
    pub rule eval() -> i64      // 規則の名前
        = term()                // 構文定義

    // 足し算を行う規則を追加 --- (*3)
    rule term() -> i64          // 規則の名前
        = v1:num() "+" v2:num() // 構文定義
        { v1 + v2 }             // アクション

    // 数値を読む規則を追加 --- (*4)
    rule num() -> i64               // 規則の名前
        = value:$(['0'..='9']+)     // 構文定義
        { value.parse().unwrap() }  // アクション
});

fn main() {
    // 足し算の計算式を実行 --- (*5)
    println!("2+5={}", calc::eval("2+5").unwrap());
    println!("8+2={}", calc::eval("8+2").unwrap());
    println!("200+50={}", calc::eval("200+50").unwrap());
}
