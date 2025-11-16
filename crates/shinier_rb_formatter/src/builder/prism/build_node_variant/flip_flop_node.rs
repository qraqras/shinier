// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/flip_flop_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_flip_flop_node(node: &FlipFlopNode<'_>, context: &mut BuildContext) -> Document {
    let left = match &node.left() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let right = match &node.right() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    Document::None
}
