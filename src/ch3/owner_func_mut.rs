// 引数の文字列を変更する関数 --- (*1)
fn add_quote(msg: &mut String) {
    msg.insert(0, '『');
    msg.push('』');
}

fn main() {
    let mut msg = String::from("強い心は病気の支えとなる");
    println!("{}", msg); // --- (*2)
    add_quote(&mut msg); // --- (*3)
    println!("{}", msg); // --- (*4)
}

