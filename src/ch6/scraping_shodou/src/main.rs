use scraper::Selector;
use std::{fs::File, io::Write};
use tokio::time;

#[tokio::main]
async fn main() {
    // 特定タイトルの作品一覧をダウンロード --- (*1)
    for title in ["温泉", "書道"] {
        download_images(title).await;
    } 
}

// 書道サイトから指定タイトルの画像をダウンロード --- (*2)
async fn download_images(title: &str) {
    let shodou_url = "https://uta.pw/shodou";
    // 指定のタイトルの作品を検索 --- (*3)
    let url = format!(
        "{}/index.php?titles&show&title={}",
        shodou_url,
        urlencoding::encode(title));
    // HTMLを取得 --- (*4)
    println!("get: {}", url);
    let html = reqwest::get(url)
        .await.unwrap()
        .text().await.unwrap();
    // HTMLをパース --- (*5)
    let doc = scraper::Html::parse_document(&html);
    // imgタグを取り出す --- (*6)
    let sel = Selector::parse(".articles img").unwrap();
    for (i, node) in doc.select(&sel).enumerate() {
        // <img src="***">のsrc属性を取り出す --- (*7)
        let src = node.value().attr("src").unwrap();
        let img_url = format!("{}/{}", shodou_url, src);
        println!("{}", img_url);
        // ダウンロードしてファイルへ保存 --- (*8)
        let filename = format!("shodou_{}_{}.png", title, i);
        let bytes = reqwest::get(img_url).await.unwrap()
                .bytes().await.unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(&bytes).unwrap();
        // 待機時間を入れる(重要) --- (*9)
        time::sleep(time::Duration::from_millis(1000)).await;
    }
}

