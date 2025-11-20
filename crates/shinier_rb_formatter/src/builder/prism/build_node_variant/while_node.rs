// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/while_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_while_node(node: &WhileNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let predicate = build_node(&node.predicate(), ctx);
    let statements = match node.statements() {
        Some(node) => Some(build_node(&node.as_node(), ctx)),
        None => None,
    };
    None
}
