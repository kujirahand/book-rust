use tokio::time;

// sec秒後にmsgを表示する非同期関数 --- (*1)
async fn say_later(sec: u64, msg: &str) {
    time::sleep(time::Duration::from_secs(sec)).await;
    println!("{}: {}", sec, msg);
}

#[tokio::main]
async fn main() {
    // spawnで並列実行する --- (*2)
    tokio::spawn(say_later(3, "毎日が宴会である。"));
    tokio::spawn(say_later(2, "陽気な人の心には..."));
    tokio::spawn(say_later(1, "苦しむ人にはどの日も悪い日で..."));
    // 並列実行の完了まで待機
    time::sleep(time::Duration::from_secs(4)).await;
    println!("------");

    // join!で並列実行する--- (*3)
    tokio::join!(
        say_later(2, "一生懸命働く充実感..."),
        say_later(3, "人にとってこれ以上の幸せはない。"),
        say_later(1, "食べ、飲み..."),
    );

}
