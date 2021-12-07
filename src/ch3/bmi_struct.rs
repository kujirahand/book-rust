// RustでBMI肥満度判定(判定表を利用)
// BMIの判定用構造体 --- (*1)
struct BmiRange {
    min: f64, // min以上
    max: f64, // max未満
    label: &'static str, // 判定ラベル
}
fn main() {
    // 身長と体重の入力 --- (*2)
    let height_cm = input("身長(cm)は? ");
    let weight = input("体重(kg)は? ");
    // BMIの計算
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    // 肥満度判定表をベクター型で用意 --- (*3)
    let bmi_list = vec![
        BmiRange { min: 0.0, max: 18.5,  label: "低体重" },
        BmiRange { min: 18.5, max: 25.0, label: "普通体重" },
        BmiRange { min: 25.0, max: 30.0, label: "肥満1度" },
        BmiRange { min: 30.0, max: 35.0, label: "肥満2度" },
        BmiRange { min: 35.0, max: 40.0, label: "肥満3度" },
        BmiRange { min: 40.0, max: 99.0, label: "肥満4度" },
    ];
    // 肥満度判定 --- (*4)
    let mut result = "不明";
    for range in bmi_list {
        if range.min <= bmi && bmi < range.max {
            result = range.label;
            break;
        }
    }
    // 結果表示
    println!("BMI={:.1},判定={}", bmi, result);
}
// 一行読み取ってf64で返す --- (*5)
fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    s.trim().parse().expect("数値変換エラー")
}
