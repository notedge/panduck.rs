use panduck_markdown::MarkdownParser;

// use notedown_ast::ASTNode;
//
// use panduck_core::Result;
// use panduck_sycamore::SycamoreConfig;
//
// mod docx;
// mod html;
// mod jupyter;
// mod markdown;
// mod rst;
//
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

const TEST_LIST1: &'static str = r#"
- a
- b
- c
"#;

#[test]
pub fn test_list() {
    let reader = MarkdownParser::default();
    let _ = reader.load_str(TEST_LIST1).unwrap();
}



//
// pub fn html_standalone_builder(parser: fn(&str) -> Result<ASTNode>) -> impl FnOnce(&str, &str) -> Result<()> {
//     move |source, target| {
//         let mut builder = SycamoreConfig::default().into_builder();
//         let ast = parser(source)?;
//         assert_eq!(builder.render_standalone(ast), target);
//         Ok(())
//     }
// }
