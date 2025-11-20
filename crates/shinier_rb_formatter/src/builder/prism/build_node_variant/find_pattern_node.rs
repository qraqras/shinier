// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/find_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_find_pattern_node(node: &FindPatternNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let constant = match &node.constant() {
        Some(node) => Some(build_node(&node, ctx)),
        None => None,
    };
    let left = build_node(&node.left().as_node(), ctx);
    let mut requireds = Vec::new();
    for node in &node.requireds() {
        requireds.push(build_node(&node, ctx));
    }
    let right = build_node(&node.right(), ctx);
    None
}
