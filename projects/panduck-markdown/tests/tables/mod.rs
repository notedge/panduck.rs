use panduck_markdown::MarkdownParser;

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
