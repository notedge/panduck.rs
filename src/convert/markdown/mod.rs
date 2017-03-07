use crate::{ToNotedown, AST};
use markdown::{Block, ListItem, Span};
use notedown_ast::SmartLink;

impl ToNotedown for Vec<Block> {
    fn to_notedown(&self) -> AST {
        AST::Statements(self.iter().map(ToNotedown::to_notedown).collect())
    }
}

impl ToNotedown for Block {
    fn to_notedown(&self) -> AST {
        let r = Default::default();
        match self {
            Block::Header(content, level) => AST::Header { level: *level, children: content.to_notedown().to_vec(), r },
            Block::Paragraph(p) => AST::Paragraph { children: p.to_notedown().to_vec(), r },
            Block::CodeBlock(_, _) => unimplemented!(),
            Block::Raw(_) => unimplemented!(),
            Block::Hr => unimplemented!(),
            Block::Blockquote(list) => AST::QuoteList { style: None, body: list.to_notedown().to_vec(), r },
            Block::OrderedList(list, _) => AST::OrderedList { head: 1, body: list.to_notedown().to_vec(), r },
            Block::UnorderedList(list) => AST::OrderlessList { body: list.to_notedown().to_vec(), r },
        }
    }
}

impl ToNotedown for Vec<Span> {
    fn to_notedown(&self) -> AST {
        let mut out = vec![];
        for node in self {
            let ast = node.to_notedown();
            match ast {
                AST::None => continue,
                AST::Statements(v) => out.extend(v),
                _ => out.push(ast),
            }
        }
        return AST::Statements(out);
    }
}

impl ToNotedown for Span {
    fn to_notedown(&self) -> AST {
        let r = Default::default();
        match self {
            Span::Break => unimplemented!(),
            Span::Text(t) => AST::Normal { inner: t.to_owned(), r },
            Span::Code(code) => {
                AST::Highlight { lang: String::from("txt"), code: code.to_owned(), inline: true, high_line: vec![], r }
            }
            Span::Link(text, url, title) => {
                let link = SmartLink::Hyperlinks {
                    from: text.into(),
                    to: url.into(),
                    alt: title.as_ref().map(Into::into),
                    bind: None,
                };
                AST::Link { inner: link, r }
            }
            Span::Image(_, _, _) => unimplemented!(),
            Span::Emphasis(_) => unimplemented!(),
            Span::Strong(_) => unimplemented!(),
        }
    }
}

impl ToNotedown for Vec<ListItem> {
    fn to_notedown(&self) -> AST {
        AST::Statements(self.iter().map(ToNotedown::to_notedown).collect())
    }
}

impl ToNotedown for ListItem {
    fn to_notedown(&self) -> AST {
        match self {
            ListItem::Simple(s) => s.to_notedown(),
            ListItem::Paragraph(p) => AST::Paragraph { children: p.to_notedown().to_vec(), r: Default::default() },
        }
    }
}