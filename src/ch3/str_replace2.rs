fn main() {
    let s = "苦しむ人にはどの日も悪い日である。";
    let s = s.replace("苦しむ人", "陽気な人");
    let s = s.replace("悪い日", "宴会");
    println!("{}", s);
}

