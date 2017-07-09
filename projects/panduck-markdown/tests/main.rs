use panduck_markdown::MarkdownParser;

mod links;
mod lists;
mod tables;

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

#[test]
pub fn test_heading() {
    let reader = MarkdownParser::default();
    let _ = reader.load_str("# Hello World!").unwrap();
    let _ = reader.load_str("## Hello World!").unwrap();
    let _ = reader.load_str("### Hello World!").unwrap();
    let _ = reader.load_str("#### Hello World!").unwrap();
    let _ = reader.load_str("##### Hello World!").unwrap();
    let _ = reader.load_str("###### Hello World!").unwrap();
    let _ = reader.load_str("####### Hello World!").unwrap();
}
