fn main() {
    let g1 = String::from("過ちを見過ごす人は美しい");
    show_message(&g1); // 参照を渡す --- (*1)
    println!("{}", g1); // ok 所有権は移動していない --- (*2)
}

fn show_message(message: &String) { // --- (*3)
    println!("{}", message);
}

