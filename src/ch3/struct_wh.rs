// 身長と体重のデータを持つ構造体 --- (*1)
struct Body {
    weight: f64,
    height: f64,
}

fn main() {
    // 構造体を初期化 --- (*2)
    let ichiro = Body{weight: 80.0, height: 165.0};
    let jiro = Body{weight: 65.0, height: 170.0};
    // 関数を呼び出す --- (*3)
    println!("Ichiro={:.1}", calc_bmi(&ichiro));
    println!("Jiro={:.1}", calc_bmi(&jiro));
}

// BMIを計算するだけの関数 --- (*4)
fn calc_bmi(body: &Body) -> f64 {
    let h = body.height / 100.0;
    body.weight / h.powf(2.0)
}

