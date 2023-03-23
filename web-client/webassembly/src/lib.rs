/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-23 11:17:06
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-23 12:08:32
 * @FilePath: /e-tech-assist/web-client/webassembly/src/lib.rs
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
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(format!("Hello {}", s).as_str());
}
