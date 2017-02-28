use crate::AST;
use orgize::{elements::Title, Element, Event, Org};

pub fn org_mode_parse(input: &str) -> AST {
    let mut nodes = vec![];
    for event in Org::parse(input).iter() {
        if let Event::Start(e) = event {
            println!("{:?}", e);
            match e {
                Element::Document { .. } => continue,
                Element::Headline { .. } => continue,
                Element::Title(t) => nodes.push(t.into()),

                Element::SpecialBlock(_) => unimplemented!(),
                Element::QuoteBlock(_) => unimplemented!(),
                Element::CenterBlock(_) => unimplemented!(),
                Element::VerseBlock(_) => unimplemented!(),
                Element::CommentBlock(_) => unimplemented!(),
                Element::ExampleBlock(_) => unimplemented!(),
                Element::ExportBlock(_) => unimplemented!(),
                Element::SourceBlock(_) => unimplemented!(),
                Element::BabelCall(_) => unimplemented!(),
                Element::Section => unimplemented!(),
                Element::Clock(_) => unimplemented!(),
                Element::Cookie(_) => unimplemented!(),
                Element::RadioTarget => unimplemented!(),
                Element::Drawer(_) => unimplemented!(),
                Element::DynBlock(_) => unimplemented!(),
                Element::FnDef(_) => unimplemented!(),
                Element::FnRef(_) => unimplemented!(),

                Element::InlineCall(_) => unimplemented!(),
                Element::InlineSrc(_) => unimplemented!(),
                Element::Keyword(_) => unimplemented!(),
                Element::Link(_) => unimplemented!(),
                Element::List(_) => unimplemented!(),
                Element::ListItem(_) => unimplemented!(),
                Element::Macros(_) => unimplemented!(),
                Element::Snippet(_) => unimplemented!(),
                Element::Text { .. } => unimplemented!(),
                Element::Paragraph { .. } => unimplemented!(),
                Element::Rule(_) => unimplemented!(),
                Element::Timestamp(_) => unimplemented!(),
                Element::Target(_) => unimplemented!(),
                Element::Bold => unimplemented!(),
                Element::Strike => unimplemented!(),
                Element::Italic => unimplemented!(),
                Element::Underline => unimplemented!(),
                Element::Verbatim { .. } => unimplemented!(),
                Element::Code { .. } => unimplemented!(),
                Element::Comment(_) => unimplemented!(),
                Element::FixedWidth(_) => unimplemented!(),

                Element::Table(_) => unimplemented!(),
                Element::TableRow(_) => unimplemented!(),
                Element::TableCell(_) => unimplemented!(),
            }
        }
    }
    return AST::Statements(nodes);
}

#[test]
fn test() {
    let ast = org_mode_parse(
        r#"
* DONE Title :tag:

#+BEGIN_SRC rust
println!("Hello");
#+END_SRC
    "#,
    );
    println!("{:?}", ast)
}

impl<'a> From<&Title<'a>> for AST {
    fn from(s: &Title<'a>) -> Self {
        let text = AST::Normal(String::from(s.raw.clone()));
        AST::Header(vec![text], s.level)
    }
}
