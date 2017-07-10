use panduck_markdown::MarkdownParser;

const TEST_TABLE1: &'static str = r#"
| Name | Age |
| ---- | --- |
| John | 20  |
| Jane | 30  |
"#;

const TEST_TABLE2: &'static str = r#"+++
toml = 1
+++
"#;

const TEST_TABLE3: &'static str = r#"---
yaml: 1
---
"#;


#[test]
pub fn test_table() {
    let reader = MarkdownParser::default();
    let _ = reader.load_str(TEST_TABLE1).unwrap();
    let _ = reader.load_str(TEST_TABLE2).unwrap();
    let _ = reader.load_str(TEST_TABLE3).unwrap();

}
