extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// JavaScriptの関数をRustで使うため --- (*1)
#[wasm_bindgen]
extern {
    // JavaScriptのalert関数をRustで使えるように
    pub fn alert(s: &str);
}

// RustでJavaScriptから使える関数を定義 --- (*2)
#[wasm_bindgen]
pub fn hello(name: &str) {
    let msg = format!("Hello, {}!", name);
    alert(&msg);
}
#[wasm_bindgen]
pub fn rust_mul(a:i32, b:i32) -> i32 {
    a * b
}

