use panduck::{parse_html};

#[test]
fn test() {
    let ast = parse_html("<p>s</p>");
    println!("{:#?}", ast.unwrap_or_default())
}
