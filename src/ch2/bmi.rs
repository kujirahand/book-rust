// RustでBMI肥満度判定ツール
fn main() {
    // 身長と体重の入力 --- (*1)
    let height_cm = input("身長(cm)は? ");
    let weight = input("体重(kg)は? ");
    // BMIの計算 --- (*2)
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI={:.1}", bmi);
    // 肥満度判定 --- (*3)
    if bmi < 18.5 { println!("低体重"); }
    else if bmi < 25.0 { println!("普通体重"); }
    else if bmi < 30.0 { println!("肥満1度"); }
    else if bmi < 35.0 { println!("肥満2度"); }
    else if bmi < 40.0 { println!("肥満3度"); }
    else { println!("肥満4度"); }
}

// 標準入力から一行読んでf64型で返す関数 --- (*4)
fn input(prompt: &str) -> f64 {
    // メッセージを表示
    println!("{}", prompt); 
    // 入力を得る --- (*5)
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    // 空白除去して数値に変換 --- (*6)
    return s.trim().parse().expect("数値変換エラー");
}

