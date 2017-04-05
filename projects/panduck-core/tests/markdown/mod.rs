use notedown_ast::ASTKind;
use panduck_core::convert::parse_common_markdown;

#[test]
fn empty() {
    let ast = parse_common_markdown("").unwrap();
    assert_eq!(ast, ASTKind::statements(vec![], None))
}

#[test]
fn test() {
    let ast = parse_common_markdown("text");
    println!("{:#?}", ast.unwrap_or_default())
}
