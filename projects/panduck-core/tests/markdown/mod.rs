use notedown_ast::AST;
use panduck::parse_markdown;

#[test]
fn empty() {
    let ast = parse_markdown("").unwrap();
    assert_eq!(ast, AST::Statements(vec![]))
}

#[test]
fn test() {
    let ast = parse_markdown("text");
    println!("{:#?}", ast.unwrap_or_default())
}
