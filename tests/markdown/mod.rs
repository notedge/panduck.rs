use notedown_from_markdown::parse_markdown;

#[test]
fn test() {
    let ast = parse_markdown("text");
    println!("{:#?}", ast)
}