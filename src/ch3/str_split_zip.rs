fn main() {
    // 郵便番号を指定
    let zipcode = "105-0011";
    
    // スライスで分割 --- (*1)
    println!("-- スライス --");
    println!("前半: {}", &zipcode[..3]);
    println!("後半: {}", &zipcode[4..]);
    
    // split_atで分割 --- (*2)
    println!("--- split_at ---");
    let (zip1, zip2) = zipcode.split_at(3);
    let (zip2, zip3) = zip2.split_at(1);
    println!("前半: {}", zip1);
    println!("記号: {}", zip2);
    println!("後半: {}", zip3);
    
    // split_offで分割 --- (*3)
    println!("-- split_off --");
    let mut zip1 = String::from(zipcode);
    let mut zip2 = zip1.split_off(3);
    let zip3 = zip2.split_off(1);
    println!("前半: {}", zip1);
    println!("記号: {}", zip2);
    println!("後半: {}", zip3);

    // splitで分割 --- (*4)
    println!("-- split --");
    let zip_a: Vec<&str> = zipcode.split('-').collect();
    println!("前半: {}", zip_a[0]);
    println!("後半: {}", zip_a[1]);
}

