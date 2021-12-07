fn main() {
    let mut g1 = String::from("過ちを見過ごす人は美しい");
    g1 = show_message(g1); // --- (*1)
    println!("{}", g1); // ok --- (*2)
}

// Stringを受け取り、Stringを返す関数 --- (*3)
fn show_message(message: String) -> String {
    println!("{}", message);
    return message;
}

