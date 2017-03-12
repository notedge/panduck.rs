use notedown_ast::AST;
use panduck::parse_jupyter;

#[test]
fn empty() {
    let ast = parse_jupyter("{}").unwrap();
    assert_eq!(ast, AST::Statements(vec![]))
}

#[test]
fn md_full() {
    let ast = parse_jupyter(include_str!("ipython.json"));
    println!("{:#?}", ast.unwrap_or_default())
}
