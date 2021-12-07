use image::{self,imageops,GenericImageView};
fn main() {
    // リサイズするサイズ
    let size = 128;
    // コマンドライン引数を得る --- (*1)
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] image_thumb imagefile");
        return;
    }
    // 入力ファイルと出力ファイルを指定 --- (*2)
    let infile = String::from(&args[1]);
    let outfile = format!("{}-thumb.png", infile);
    println!("input: {}", infile);
    println!("output: {}", outfile);
    // 画像ファイルを読み込む --- (*3)
    let mut img = image::open(infile)
        .expect("画像ファイルが読めません");
    // 画像サイズを得る --- (*4)
    let dim = img.dimensions();
    // 正方形に切り取る --- (*5)
    let w = if dim.0 > dim.1 {dim.1} else {dim.0};
    let mut img2 = imageops::crop(&mut img,
        (dim.0-w)/2, (dim.1-w)/2, w, w).to_image();
    // 指定サイズにリサイズ --- (*6)
    let img3 = imageops::resize(&mut img2, size, size, 
        imageops::Lanczos3);
    // ファイルへ保存 --- (*7)
    img3.save(outfile).unwrap();
}

