use panduck::parse_markdown;

#[test]
fn test() {
    let ast = parse_markdown("text");
    println!("{:#?}", ast.unwrap_or_default())
}
