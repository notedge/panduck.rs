use crate::{builder::SycamoreBuilder, traits::IntoSycamore};
use notedown_ast::ASTNodes;
use sycamore::prelude::GenericNode;

pub fn error_inline<G: GenericNode>(msg: &str) -> G {
    let node: G = GenericNode::element("span");
    node.set_class_name("debug-error");
    node.update_inner_text(msg);
    return node;
}

pub fn error_block<G: GenericNode>(msg: &str) -> G {
    let node: G = GenericNode::element("p");
    node.set_class_name("debug-error");
    node.update_inner_text(msg);
    return node;
}

pub fn push_nodes<G>(node: &G, children: ASTNodes, ctx: &SycamoreBuilder)
where
    G: GenericNode,
{
    for i in children {
        node.append_child(&i.value.into_sycamore(ctx))
    }
}

pub fn unwrap_inner<G: GenericNode>(node: G) -> G {
    match node.first_child() {
        Some(s) => s,
        #[cfg(debug_assertions)]
        None => panic!("Illegal HTML content"),
        #[cfg(not(debug_assertions))]
        None => GenericNode::marker(),
    }
}
