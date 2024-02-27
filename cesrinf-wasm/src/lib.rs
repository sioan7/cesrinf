use wasm_bindgen::prelude::*;

pub mod domain;
pub mod error;
pub mod parser;

#[wasm_bindgen]
pub fn constant() -> usize {
    6969
}
