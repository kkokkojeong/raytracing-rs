// https://doc.rust-kr.org/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
pub mod ray;
pub mod raytracer;
pub mod sphere;
pub mod hit;
pub mod light;
pub mod state;
pub mod wgpu_texture_util;
pub mod triangle;
pub mod square;
pub mod texture;
/*
* WebAssembly (WASM) 시 도전!
*/
// #[cfg(target_arch="wasm32")]
// use wasm_bindgen::prelude::*;
//
// #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
// pub fn run() {
//
// }