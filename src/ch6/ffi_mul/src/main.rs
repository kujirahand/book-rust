// C言語で書いたライブラリの名前を指定 --- (*1)
#[link(name="mycalc", kind="static")]
extern "C" {
    // C言語で定義した関数を指定
    fn mul(a: isize, b: isize) -> isize;
}

fn main() {
    // C言語の関数を呼び出す --- (*2)
    unsafe {
        let n = mul(30, 5);
        println!("{}", n);
        let n = mul(8, 80);
        println!("{}", n);
    }
}

