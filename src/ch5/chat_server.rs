use std::net::{TcpListener, TcpStream};
use std::io::{BufReader,BufRead,Write};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // サーバーのアドレスとポート番号を指定 --- (*1)
    let server_addr = "127.0.0.1:8888";
    // スレッド間通信を用意 --- (*2)
    let (tx, rx) = mpsc::channel::<String>();
    // クライアント一覧を覚えておく --- (*3)
    let mut clients: Vec<TcpStream> = Vec::new();
    
    // サーバーを起動 --- (*4)
    let server = TcpListener::bind(server_addr)
      .expect("サーバーの起動に失敗");
    server.set_nonblocking(true).expect("利用不可");
    println!("{}でサーバーを起動しました。", server_addr);
    
    // メインループ --- (*5)
    loop {
        // クライアントを待ち受け --- (*6)
        if let Ok((client, addr)) = server.accept() {
            println!("クライアントが接続: {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }
        // スレッド間通信の待ち受け --- (*7)
        if let Ok(msg) = rx.try_recv() {
            println!("全員に送信: {}", msg.trim());
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}
// クライアントのために受信用スレッドを開始 --- (*8)
fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        // メッセージを待つ -- (*9)
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            // 受信内容をメインスレッドに送信 --- (*10)
            if n > 0 { tx.send(msg).unwrap(); }
        }
        thread::sleep(Duration::from_millis(100));
    });
}
// 全てのクライアントにメッセージを送信 --- (*11)
fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];
    for mut socket in clients.into_iter() {
        // 文字列をバイト列に変換して送信 --- (*12)
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("送信エラー: {}", e);
            continue;
        }
        collector.push(socket); // 所有権を回収
    }
    collector // 所有権を戻す
}
