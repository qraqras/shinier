// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/multi_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::multi_target_node::{layout_multi_target_node, LayoutParamMultiTargetNode};

fn build_multi_target_node(node: &MultiTargetNode<'_>, context: &mut BuildContext) -> Document {
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
    };
    layout_multi_target_node(&LayoutParamMultiTargetNode { lefts, rest, rights })
}
