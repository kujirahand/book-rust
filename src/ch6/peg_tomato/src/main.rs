mod parser;
mod runner;
mod node;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] peg_tomato file.tomato");
        return;
    }
    // ファイルを開く
    let filename = &args[1];
    let src = fs::read_to_string(filename).unwrap();
    runner::run(&src);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        assert_eq!(runner::run("print 32"), 32);
        assert_eq!(runner::run("print 1+2*3"), 7);
        assert_eq!(runner::run(
            "a=3;if a==3 { print 1 } else { print 0 }"), 1);
        assert_eq!(runner::run(
            "a=0;for i=1 to 10 { a=a+i }; print a"), 55);
        assert_eq!(runner::run("print \"abc\""), 0);
    }
}
