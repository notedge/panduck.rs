use notedown_ast::ASTKind;
use panduck_core::convert::parse_html;

#[test]
fn empty() {
    let ast = parse_html("").unwrap();
    assert_eq!(ast, ASTKind::statements(vec![], None))
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
    assert_eq!(format!("{:#?}", ast.unwrap()), include_str!("md_fragment.yaml"))
}

#[test]
fn md_full() {
    let ast = parse_html(include_str!("md_full.html"));
    assert_eq!(format!("{:#?}", ast.unwrap()), include_str!("md_full.yaml"))
}
