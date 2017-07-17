use panduck_org_mode::MarkdownParser;

const TEST_LIST1: &'static str = r#"
- a
- b
- c
"#;

const TEST_LIST2: &'static str = r#"
* a
* b
* c
"#;

const TEST_LIST3: &'static str = r#"
> a
> b
> c
"#;

const TEST_LIST4: &'static str = r#"
> a
> - a.a
> - a.b
>   - a.b.a
>   - a.b.c
"#;


#[test]
pub fn test_list() {
    let reader = MarkdownParser::default();
    let _ = reader.load_str(TEST_LIST1).unwrap();
    let _ = reader.load_str(TEST_LIST2).unwrap();
    let _ = reader.load_str(TEST_LIST3).unwrap();
    let _ = reader.load_str(TEST_LIST4).unwrap();
}
