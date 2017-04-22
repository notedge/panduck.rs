use crate::traits::IntoSycamore;
use notedown_ast::ASTNode;
use panduck_html::HTMLConfig;
use sycamore::{render_to_string, view::View, SsrNode};

pub struct SycamoreBuilder {
    pub config: HTMLConfig,
}

impl Default for SycamoreBuilder {
    fn default() -> Self {
        Self { config: Default::default() }
    }
}

impl SycamoreBuilder {
    pub fn render(&self, ast: ASTNode) -> String {
        let view = View::<SsrNode>::new_node(ast.into_sycamore(self));
        render_to_string(|| view)
    }
}
