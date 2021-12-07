use tokio::time;

#[tokio::main]
async fn main() {
    // 非同期処理を連続で実行 --- (*1)
    for i in 1..=3 {
        println!("#{}を開始", i);
        // 非同期関数を実行して結果を得る --- (*2)
        let s = read_longtime().await;
        println!("{}", s);
        // 非同期処理はブロックでも記述可能 --- (*3)
        let s = async {
            time::sleep(time::Duration::from_secs(1)).await;
            String::from("長い読み込み完了(block)")
        }.await;
        println!("{}", s);
    }
}

// 実行に時間がかかる関数 --- (*4)
async fn read_longtime() -> String {
    time::sleep(time::Duration::from_secs(1)).await;
    String::from("長い読み込み完了(fn)")
}

