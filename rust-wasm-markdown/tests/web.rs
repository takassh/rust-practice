//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

extern crate rust_wasm_markdown;
use rust_wasm_markdown::{ToHtml, *};

#[wasm_bindgen_test]
fn text_to_p() {
    let html = ToHtml("hello world");

    assert_eq!(html, "<p>hello world</p>\n");
}
