// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/find_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_find_pattern_node(node: &FindPatternNode<'_>, context: &mut BuildContext) -> Document {
    let constant = match &node.constant() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let left = build_node(&node.left().as_node(), context);
    let mut requireds = Vec::new();
    for node in &node.requireds() {
        requireds.push(build_node(&node, context));
    }
    let right = build_node(&node.right(), context);
    Document::None
}
