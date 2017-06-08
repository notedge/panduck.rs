use notedown_ast::ASTNode;
use sycamore::{prelude::GenericNode, render_to_string, view::View, SsrNode};

use panduck_html::HTMLConfig;

use crate::traits::IntoSycamore;

pub use self::{config::SycamoreConfig, context::SycamoreContext};

mod config;
mod context;

#[derive(Default)]
pub struct SycamoreBuilder {
    pub cfg: SycamoreConfig,
    pub ctx: SycamoreContext,
}

impl SycamoreBuilder {
    /// the html fragment
    pub fn render(&self, ast: ASTNode) -> String {
        let view = View::<SsrNode>::new_node(ast.into_sycamore(self));
        render_to_string(|| view)
    }
    /// a complete html
    pub fn render_standalone(&self, ast: ASTNode) -> String {
        let html = SsrNode::element("html");
        html.append_child(&self.html_head());
        html.append_child(&ast.into_sycamore(self));
        render_to_string(|| View::new_node(html))
    }

    fn html_head<G: GenericNode>(&self) -> G {
        let head: G = GenericNode::element("head");
        head.append_child(&{
            let meta: G = GenericNode::element("meta");
            meta.set_attribute("charset", "UTF-8");
            meta
        });
        head.append_child(&{
            let meta: G = GenericNode::element("meta");
            meta.set_attribute("name", "viewport-8");
            meta.set_attribute("content", "'width=device-width, initial-scale=1.0'");
            meta
        });
        head.append_child(&{
            let meta: G = GenericNode::element("link");
            meta.set_attribute("rel", "stylesheet");
            meta.set_attribute("href", "https://latex.now.sh/style.css");
            meta
        });
        return head;
    }
}
