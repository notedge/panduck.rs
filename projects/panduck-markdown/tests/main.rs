use panduck_markdown::MarkdownParser;

mod lists;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_normal() {
    let reader = MarkdownParser::default();
    let _ = reader.load_str("Hello World!").unwrap();
}

#[test]
pub fn test_styles() {
    let reader = MarkdownParser::default();
    let _ = reader.load_str("*Hello World!*").unwrap();
    let _ = reader.load_str("**Hello World!**").unwrap();
    let _ = reader.load_str("***Hello World!***").unwrap();

    let _ = reader.load_str("_Hello World!_").unwrap();
    let _ = reader.load_str("__Hello World!__").unwrap();
    let _ = reader.load_str("___Hello World!___").unwrap();

    let _ = reader.load_str("~Hello World!~").unwrap();
    let _ = reader.load_str("~~Hello World!~~").unwrap();
    let _ = reader.load_str("~~~Hello World!~~~").unwrap();

    let _ = reader.load_str("`Hello World!`").unwrap();
    let _ = reader.load_str("``Hello World!``").unwrap();
    let _ = reader.load_str("```Hello World!```").unwrap();

    let _ = reader.load_str("$Hello World!$").unwrap();
    let _ = reader.load_str("$$Hello World!$$").unwrap();
    let _ = reader.load_str("$$$Hello World!$$$").unwrap();
}

const TEST_TABLE1: &'static str = r#"
| Name | Age |
| ---- | --- |
| John | 20  |
| Jane | 30  |
"#;

#[test]
pub fn test_table() {
    let reader = MarkdownParser::default();
    let _ = reader.load_str(TEST_TABLE1).unwrap();
}
