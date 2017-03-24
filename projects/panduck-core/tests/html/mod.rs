use panduck_core::{parse_html, AST};

#[test]
fn empty() {
    let ast = parse_html("").unwrap();
    assert_eq!(ast, AST::statements(vec![]))
}

#[test]
fn simple() {
    let ast = parse_html("<p>text</p>");
    println!("{:#?}", ast.unwrap_or_default())
}

#[test]
fn nested() {
    let ast = parse_html("<p>text<i>i</i><b>b</b></p>");
    println!("{:#?}", ast.unwrap_or_default())
}

#[test]
fn md_fragment() {
    let ast = parse_html(include_str!("md_fragment.html"));
    println!("{:#?}", ast.unwrap_or_default())
}

#[test]
fn md_full() {
    let ast = parse_html(include_str!("md_full.html"));
    println!("{:#?}", ast.unwrap_or_default())
}
