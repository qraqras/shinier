// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/for_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_for_node(node: &ForNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let index = build_node(&node.index(), ctx);
    let collection = build_node(&node.collection(), ctx);
    let statements = match node.statements() {
        Some(node) => Some(build_node(&node.as_node(), ctx)),
        None => None,
    };
    None
}
