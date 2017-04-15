use notedown_ast::ASTKind;
use panduck_core::convert::parse_common_markdown;

#[test]
fn empty() {
    let ast = parse_common_markdown("").unwrap();
    assert_eq!(ast, ASTKind::statements(vec![], None))
}

#[test]
fn test3() {
    let ast = parse_common_markdown(include_str!("markdown-it.md"));
    assert_eq!(format!("{:#?}", ast.unwrap()), include_str!("markdown-it-cmd.yaml"))
}
