extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod cipher_str;

#[wasm_bindgen]
pub fn encrypt(password: &str, data: &str) -> String {
    cipher_str::encrypt(password, data)
}
#[wasm_bindgen]
pub fn decrypt(password: &str, data: &str) -> String {
    cipher_str::decrypt(password, data)
}

