use panduck_markdown::MarkdownParser;

const TEST_IMAGE: &'static str = r#"
![image-reference]
![image-title](link)
"#;

const TEST_LINK: &'static str = r#"
[name](link)
[[double]]

[block]
[^a]

[^a]: b
[a]: b
"#;


#[test]
pub fn test_table() {
    let reader = MarkdownParser::default();
    let _ = reader.load_str(TEST_IMAGE).unwrap();
    let _ = reader.load_str(TEST_LINK).unwrap();
}


