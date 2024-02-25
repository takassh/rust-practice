mod utils;

use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;


#[wasm_bindgen]
pub fn to_html(input: &str) -> String {
    set_panic_hook();
    // Create parser with example Markdown text.
    let parser = pulldown_cmark::Parser::new(input);

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}
