use std::io;
fn main() {
    // 計算用のスタック --- (*1)
    let mut stack: Vec<f64> = vec![];
    // 標準入力から数式を得る --- (*2)
    println!("RPN:");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("入力エラー");
    // 式を空白で分割して繰り返し計算 --- (*3)
    let tokens = s.split_whitespace();
    for tok in tokens {
        let t = tok.trim();
        // 数値ならスタックにPUSH --- (*4)
        match t.parse::<f64>() {
            Ok(v) => { stack.push(v); continue; },
            Err(_) => 0.0,
        };
        // 演算子なら2回POPして計算結果をPUSH --- (*5)
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        match t {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "*" => stack.push(a * b),
            "/" => stack.push(a / b),
            _ => panic!("未定義の演算子:{}",t),
        }
    }
    // 結果を表示 --- (*6)
    println!("{}", stack.pop().unwrap());
}

