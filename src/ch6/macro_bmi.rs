// 肥満判定を行うマクロ --- (*1)
macro_rules! bmi_select {
    // パターンを指定
    ( $bmi:expr; $( $label:expr => $range:expr );+) => {{
        // マクロのデフォルト戻り値
        let mut result = "error";
        // 繰り返し --- (*2)
        $(
            if $range.start <= $bmi && $bmi < $range.end {
                result = $label;
            }
        )+
        result
    }};
}

fn main() {
    // 身長と体重の指定
    let h: f32 = 158.0;
    let w: f32 = 63.0;
    let bmi = w / (h / 100.0).powf(2.0);
    // BMIを判定するマクロを実行 --- (*3)
    let label = bmi_select![
        bmi;
        "低体重" =>  0.0..18.5;
        "普通"   => 18.5..25.0;
        "肥満1"  => 25.0..30.0;
        "肥満2"  => 30.0..35.0;
        "肥満3"  => 35.0..40.0;
        "肥満4"  => 40.0..99.9];
    println!("判定: {}", label);
}

