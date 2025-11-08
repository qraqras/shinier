// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/hash_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::hash_pattern_node::{layout_hash_pattern_node, LayoutParamHashPatternNode};

fn build_hash_pattern_node(node: &HashPatternNode<'_>, context: &mut BuildContext) -> Document {
    let constant = match node.constant() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut elements = Vec::new();
    for node in &node.elements() {
        elements.push(build_node(&node, context));
    }
    let rest = match node.rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_hash_pattern_node(&LayoutParamHashPatternNode { constant, elements, rest })
}
