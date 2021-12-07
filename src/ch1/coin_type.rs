fn main() {
    // 期待する金額 --- (*1)
    let price: i64 = 3950;
    // コインの保有数を指定 --- (*2)
    let count500: i64 = 10;
    let count100: i64 = 3;
    let count50: i64  = 10;
    // 繰り返し計算して答えを探す
    for i500 in 0..(count500+1) {
        for i100 in 0..(count100+1) {
            for i50 in 0..(count50+1) {
                // 総額を計算 --- (*3)
                let total: i64 = 
                    i50 * 50 + i100 * 100 + i500 * 500;
                // 総額が期待する金額になるか
                if price == total {
                    println!("500円x{}+100円x{}+50円x{}={}",
                        i500, i100, i50, total);
                }
            }
        }
    }
}

