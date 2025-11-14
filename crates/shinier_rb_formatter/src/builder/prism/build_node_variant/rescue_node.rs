// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/rescue_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::rescue_node::{layout_rescue_node, LayoutParamRescueNode};

pub fn build_rescue_node(node: &RescueNode<'_>, context: &mut BuildContext) -> Document {
    let mut expressions = Vec::new();
    for node in &node.exceptions() {
        expressions.push(build_node(&node, context));
    }
    let reference = match &node.reference() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let subsequent = match &node.subsequent() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_rescue_node(&LayoutParamRescueNode { expressions, reference, statements, subsequent })
}
