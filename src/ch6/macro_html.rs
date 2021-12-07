// HTML構造を再帰的に出力するマクロの定義 --- (*1)
macro_rules! out_html {
    // 引数なしの場合 --- (*2)
    () => {()};
    // 引数が1つだけの場合 --- (*3)
    ($e:tt) => {print!("{}", $e)};
    // タグ [ 内側 ] 続きを指定した場合 --- (*4)
    ($tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        print!("<{}>", stringify!($tag));
        out_html!($($inner)*);
        println!("</{}>", stringify!($tag));
        out_html!($($rest)*);
    }};
}
fn main() {
    // マクロを利用してHTML構造を手軽に出力 --- (*5)
    out_html!(
        html [
            head[title["test"]]
            body[
                h1["test"]
                p ["This is test."]
            ]
        ]
    );
}

