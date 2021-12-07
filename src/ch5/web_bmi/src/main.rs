use actix_web::{
    web, App, HttpServer, HttpRequest, HttpResponse, Error};
use serde::{Deserialize, Serialize};

// アドレスとポートの指定 --- (*1)
const SERVER_ADDR: &str = "127.0.0.1:8888";

// Actix Webのメイン関数 --- (*2)
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    // サーバーを起動 --- (*3)
    HttpServer::new(|| {
        // ルーティングを指定 --- (*4)
        App::new()
            .route("/", web::get().to(index))
            .route("/calc", web::get().to(calc))
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

// 実行される関数 --- (*5)
async fn index(_: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
       .content_type("text/html; charset=utf-8")
       .body(format!("{}{}{}{}{}{}",
            "<html><body><h1>BMI判定</h1>",
            "<form action='calc'>",
            "身長: <input name='height' value='160'><br>",
            "体重: <input name='weight' value='70'><br>",
            "<input type='submit' value='送信'>",
            "</form></body></html>")))
}

// 入力フォームのデータを定義 --- (*6)
#[derive(Serialize, Deserialize, Debug)]
pub struct FormBMI {
    height: f64,
    weight: f64,
}

// "/calc"にアクセスがあったときBMIを計算 --- (*7)
async fn calc(q: web::Query<FormBMI>) -> 
  Result<HttpResponse, Error> {
    // フォームからパラメータをとれたか確認
    println!("{:?}", q);
    // BMIを計算
    let h = q.height / 100.0;
    let bmi = q.weight / (h * h);
    let per = (bmi / 22.0) * 100.0;
    // 結果を表示
    Ok(HttpResponse::Ok()
       .content_type("text/html; charset=utf-8")
       .body(format!(
            "<h3>BMI={:.1}, 乖離率={:.0}%</h3>", bmi, per)))
}

