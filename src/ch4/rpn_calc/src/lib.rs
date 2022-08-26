// ドキュメントを記述 --- (*1)
//! # RPN Calc
//! Reverse Polish notation (RPN) Calc.
//! # Example
//! ```
//! let src = String::from("1 2 + 3 * ");
//! let a = rpn_calc::eval(src).unwrap();
//! println!("{}", a); // →9
//! ```

/// RPN式を計算する関数
pub fn eval(src: String) -> Result<f64, String> {
    // 空白でトークンを区切る
    let tokens = src.split_whitespace();
    let mut stack:Vec<f64> = vec![];
    // 繰り返し要素を計算 --- (*2)
    for tok in tokens {
        let t = tok.trim();
        if t == "" { continue; }
        // 数値ならスタックにPUSH
        match t.parse::<f64>() {
            Ok(v) => { stack.push(v); continue; },
            Err(_) => 0.0,
        };
        // 演算子なら2回POPして計算結果をPUSH
        let b = stack.pop().unwrap_or(0.0);
        let a = stack.pop().unwrap_or(0.0);
        match t {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "*" => stack.push(a * b),
            "/" => stack.push(a / b),
            "%" => stack.push(a % b),
            _ => return Err(format!("invalid operator: {}", t)),
        }
    }
    // 結果を返す --- (*3)
    if stack.len() == 0 { return Err(format!("no result")); }
    if stack.len() > 1 { 
        return Err(format!("too many value in stack")); 
    } 
    Ok(stack.pop().unwrap_or(0.0))
}

/// より手軽に使う
pub fn eval_str(src: &str) -> String {
    match eval(String::from(src)) {
        Ok(v) => format!("{}", v),
        Err(e) => format!("[ERROR] {}", e),
    }
}

// ライブラリを利用するテストを記述 --- (*4)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // eval を使う
        assert_eq!(eval("1 3 +".to_string()), Ok(4.0));
        assert_eq!(eval("2 3 *".to_string()), Ok(6.0));
        assert_eq!(eval("6 3 /".to_string()), Ok(2.0));
        assert_eq!(eval("6 3 - 1 -".to_string()), Ok(2.0));
        // eval_str を使う
        assert_eq!(eval_str("1 2 3 + +"), "6".to_string());
        assert_eq!(eval_str("1 2 3 * +"), "7".to_string());
    }
}

