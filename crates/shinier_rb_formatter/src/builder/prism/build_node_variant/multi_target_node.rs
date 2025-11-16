// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/multi_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_multi_target_node(node: &MultiTargetNode<'_>, context: &mut BuildContext) -> Document {
    let mut lefts = Vec::new();
    for node in &node.lefts() {
        lefts.push(build_node(&node, context));
    }
    let rest = match &node.rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut rights = Vec::new();
    for node in &node.rights() {
        rights.push(build_node(&node, context));
    }
    Document::None
}
