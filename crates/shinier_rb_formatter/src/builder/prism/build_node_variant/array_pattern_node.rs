// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/array_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_array_pattern_node(node: &ArrayPatternNode<'_>, context: &mut BuildContext) -> Document {
    let constant = match &node.constant() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut requireds = Vec::new();
    for node in &node.requireds() {
        requireds.push(build_node(&node, context));
    }
    let rest = match &node.rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut posts = Vec::new();
    for node in &node.posts() {
        posts.push(build_node(&node, context));
    }
    Document::None
}
