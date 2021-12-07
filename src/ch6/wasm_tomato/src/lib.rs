extern crate wasm_bindgen;
mod runner;
mod parser;
mod node;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tomato_run(src: &str) -> String {
    runner::run(src)
}

