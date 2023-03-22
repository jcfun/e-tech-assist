/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-22 19:14:55
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-23 01:08:09
 * @FilePath: /e-tech-assist/web-client/src/lib.rs
 * @Description: 
 */
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // alert(format!("Hello, {}!", s).as_str());
    alert("666666")
}
