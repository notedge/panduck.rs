use std::borrow::Cow;

use pretty::{RcAllocator, RcDoc};

use crate::PrettyPrint;

/// nothing output
pub fn nil<'a>() -> PrettyPrint<'a> {
    RcDoc::nil()
}

/// text
pub fn text<'a, S: Into<Cow<'a, str>>>(s: S) -> PrettyPrint<'a> {
    RcDoc::text(s)
}

/// skip one line
pub fn block_break<'a>() -> PrettyPrint<'a> {
    RcDoc::text("\n\n")
}

/// skip one line
pub fn newline<'a>() -> PrettyPrint<'a> {
    RcDoc::text("\n")
}

/// - `NIL` if inline
/// - `\n` if break
pub fn nil_or_newline<'a>() -> PrettyPrint<'a> {
    RcDoc::hardline().flat_alt(RcDoc::nil())
}

/// - `\s` if inline
/// - `\n` if break line
pub fn space_or_newline<'a>() -> PrettyPrint<'a> {
    RcDoc::hardline().flat_alt(RcDoc::nil())
}

/// ```js
/// [1, 2, 3]
///
/// <
///  str
///  end
/// />
///
/// ```
///
///
pub struct OpenClosedGroup {
    ident: usize,
    inline: String,
    newline: String,
    inline_end_mark: bool,
    block_end_mark: bool,
}

impl Default for OpenClosedGroup {
    fn default() -> Self {
        Self {
            //
            ident: 4,
            inline: String::from(", "),
            newline: String::from(","),
            inline_end_mark: false,
            block_end_mark: true,
        }
    }
}

impl OpenClosedGroup {
    pub fn set_mark<S: Into<String>>(&mut self, inline: S, multiline: S) {
        self.inline = inline.into();
        self.newline = multiline.into();
    }

    pub fn set_end_mark(&mut self, inline: bool, multiline: bool) {
        self.inline_end_mark = inline;
        self.block_end_mark = multiline;
    }
    pub fn set_indent(&mut self, indent: usize) -> &mut OpenClosedGroup {
        self.ident = indent;
        self
    }
}

impl OpenClosedGroup {
    #[inline]
    pub fn pretty_print<'i, T, I>(&self, start: T, end: T, items: I, width: usize)
    where
        T: Into<Cow<'i, str>>,
        I: IntoIterator,
        I::Item: pretty::Pretty<'i, RcAllocator, ()>,
    {
        println!("{}", self.pretty_render(start, end, items, width))
    }
    #[inline]
    pub fn pretty_render<'i, T, I>(&self, start: T, end: T, items: I, width: usize) -> String
    where
        T: Into<Cow<'i, str>>,
        I: IntoIterator,
        I::Item: pretty::Pretty<'i, RcAllocator, ()>,
    {
        self.build(start, end, items).pretty(width).to_string()
    }

    pub fn build<'i, T, I>(&self, start: T, end: T, items: I) -> PrettyPrint<'i>
    where
        T: Into<Cow<'i, str>>,
        I: IntoIterator,
        I::Item: pretty::Pretty<'i, RcAllocator, ()>, // life time of input items
    {
        let inline = RcDoc::as_string(&self.inline);
        let newline = RcDoc::as_string(&self.newline).append(RcDoc::hardline());
        let separator = newline.flat_alt(inline);
        let middle = RcDoc::intersperse(items, separator);
        let mut middle = nil_or_newline().append(middle);
        if self.inline_end_mark && self.block_end_mark {
            let mark = RcDoc::as_string(&self.newline);
            middle = middle.append(mark);
        }
        else if self.block_end_mark {
            let mark = RcDoc::as_string(&self.newline).flat_alt(RcDoc::nil());
            middle = middle.append(mark);
        }
        else {
            // do nothing
        }
        let middle = middle.nest(self.ident as isize).append(nil_or_newline()).group();
        text(start).append(middle).append(text(end))
    }
}
