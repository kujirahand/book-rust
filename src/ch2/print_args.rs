use std::env;

fn main() {
    let args = env::args();
    for arg in args {
        println!("{}", arg);
    }
}

