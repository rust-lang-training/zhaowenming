use comrak::{formats::Html, ComrakOptions, markdown_to_html};
#[wasm_bindgen]
pub fn markdown_to_html_content() {
    let markdown = r#"
# Markdown to HTML Conversion

This is an example of how to use the `comrak` crate to convert Markdown to HTML.

## Features:

- Fast and robust Markdown parsing
- CommonMark compliance
- Extensible with support for various Markdown extensions
- Customizable HTML output
"#;

    let html = markdown_to_html(markdown, &ComrakOptions::default());
    println!("{}", html);
}
