// BMIと肥満度表示する関数 --- (*1)
fn print_bmi(height: f32, weight: Option<f32>) {
    // weightに値があればBMIを求めてOption型で返す --- (*2)
    let bmi:Option<f32> = match weight {
        Some(w) => Some(w / (height / 100.0).powf(2.0)),
        None => None,
    };
    // BMIの値に応じて判定 --- (*3)
    let msg = match bmi {
        Some(n) if n < 18.5 => "低体重",
        Some(n) if n < 25.0 => "普通体重",
        Some(n) if n < 30.0 => "肥満1度",
        Some(n) if n < 35.0 => "肥満2度",
        Some(n) if n < 40.0 => "肥満3度",
        Some(_) => "肥満4度",
        None => "測定不能",
    };
    // 判定結果を表示 --- (*4)
    println!("BMI={:.1}, 判定={}", bmi.unwrap_or(0.0), msg);
}
// いろいろな入力を与える --- (*5)
fn main() {
    let height = 162.3;
    print_bmi(height, Some(48.0));
    print_bmi(height, Some(72.3));
    print_bmi(height, None);
}

