use super::*;

use fluent_syntax::ast::{
    Attribute, Comment, Entry, Expression, InlineExpression, Message, NamedArgument, Pattern, PatternElement, Resource, Term, VariantKey,
};

pub trait PrettyPrint {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result;
}

impl PrettyPrint for Resource<&str> {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result {
        for i in &self.body {
            PrettyPrint::pretty_print(i, f)?
        }
        Ok(())
    }
}

impl PrettyPrint for Entry<&str> {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result {
        match self {
            Entry::Message(s) => PrettyPrint::pretty_print(s, f),
            Entry::Term(s) => PrettyPrint::pretty_print(s, f),
            Entry::Comment(s) => f.write_new_indent(s, "# "),
            Entry::GroupComment(s) => PrettyPrint::pretty_print(s, f),
            Entry::ResourceComment(s) => {
                f.write_new_indent(s, "### ")?;
                Ok(f.write_new_line())
            }
            Entry::Junk { .. } => {
                unimplemented!()
            }
        }
    }
}

impl PrettyPrint for Message<&str> {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result {
        // self.comment
        if let Some(comment) = &self.comment {
            f.write_new_indent(comment, "# ")?
        }
        // self.id
        write!(f, "{} = ", self.id.name)?;
        // self.value
        let mut one_line = true;
        if let Some(s) = &self.value {
            one_line = text_is_one_line(s);
            match one_line {
                true => PrettyPrint::pretty_print(s, f)?,
                false => {
                    f.write_new_line();
                    f.write_new_indent(s, &f.tab())?;
                }
            }
        }
        // self.attributes
        let mut indent = f.indent.to_owned() + &f.tab();
        f.swap_indent(&mut indent);
        for item in &self.attributes {
            f.write_new_line();
            item.pretty_print(f)?;
        }
        f.swap_indent(&mut indent);
        // end
        Ok(())
    }
}

impl PrettyPrint for Term<&str> {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl PrettyPrint for Pattern<&str> {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result {
        for item in &self.elements {
            match item {
                PatternElement::TextElement { value } => f.write_str(value)?,
                PatternElement::Placeable { expression } => PrettyPrint::pretty_print(expression, f)?,
            }
        }
        Ok(())
    }
}

fn text_is_one_line(text: &Pattern<&str>) -> bool {
    for i in &text.elements {
        match i {
            PatternElement::TextElement { value } => {
                if value.contains('\n') {
                    return false;
                }
            }
            PatternElement::Placeable { expression: Expression::Select { .. } } => return false,
            _ => continue,
        }
    }
    return true;
}

impl PrettyPrint for InlineExpression<&str> {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result {
        match self {
            InlineExpression::StringLiteral { .. } => {
                unimplemented!()
            }
            InlineExpression::NumberLiteral { value } => {
                write!(f, "{}", value)
            }
            InlineExpression::FunctionReference { id, arguments } => {
                write!(f, "{}", id.name)?;
                write!(f, "(")?;
                let length = arguments.positional.len() + arguments.named.len();
                let mut this = 0;

                for item in &arguments.positional {
                    this += 1;

                    PrettyPrint::pretty_print(item, f)?;
                    if this != length {
                        write!(f, ", ")?
                    }
                }
                for NamedArgument { name, value } in &arguments.named {
                    this += 1;
                    write!(f, "{} = ", name.name)?;
                    PrettyPrint::pretty_print(value, f)?;
                    if this != length {
                        write!(f, ", ")?
                    }
                }
                write!(f, ")")
            }
            InlineExpression::MessageReference { .. } => {
                unimplemented!()
            }
            InlineExpression::TermReference { id, attribute, arguments } => {
                f.write_str(id.name)?;
                match attribute {
                    Some(s) => writeln!(f, "{:#?}", s)?,
                    None => {}
                }
                match arguments {
                    Some(s) => writeln!(f, "{:#?}", s)?,
                    None => {}
                }
                Ok(())
            }
            InlineExpression::VariableReference { id } => {
                write!(f, "${}", id.name)
            }
            InlineExpression::Placeable { .. } => {
                unimplemented!()
            }
        }
    }
}

impl PrettyPrint for Expression<&str> {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result {
        match self {
            Expression::Select { selector, variants } => {
                write!(f, "{{")?;
                f.write_new_line();
                f.write_with_indent("");
                PrettyPrint::pretty_print(selector, f)?;
                write!(f, " -> ")?;
                f.write_new_line();
                for item in variants {
                    match item.default {
                        true => f.write_with_indent("   *"),
                        false => f.write_with_indent("    "),
                    }
                    write!(f, "[")?;
                    match item.key {
                        VariantKey::Identifier { name } => f.write_str(name)?,
                        VariantKey::NumberLiteral { value } => f.write_str(value)?,
                    }
                    write!(f, "] ")?;
                    PrettyPrint::pretty_print(&item.value, f)?;
                    f.write_new_line()
                }
                f.write_with_indent("}");
            }
            Expression::Inline(s) => {
                write!(f, "{{")?;
                write!(f, "{}", " ".repeat(f.config.inline_space as usize))?;
                PrettyPrint::pretty_print(s, f)?;
                write!(f, "{}", " ".repeat(f.config.inline_space as usize))?;
                write!(f, "}}")?;
            }
        }
        Ok(())
    }
}

impl PrettyPrint for Attribute<&str> {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result {
        f.write_with_indent(".");
        write!(f, "{} = ", self.id.name)?;
        self.value.pretty_print(f)
    }
}

impl PrettyPrint for Comment<&str> {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result {
        for i in &self.content {
            f.write_with_indent(i);
            f.write_new_line()
        }
        Ok(())
    }
}

impl PrettyPrint for &str {
    fn pretty_print(&self, f: &mut FluentFormatter) -> fmt::Result {
        Ok(f.buffer.push_str(self))
    }
}
