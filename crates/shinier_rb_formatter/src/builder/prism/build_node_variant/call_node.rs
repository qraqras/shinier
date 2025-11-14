// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/call_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::call_node::{layout_call_node, LayoutParamCallNode};

pub fn build_call_node(node: &CallNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match node.block() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_call_node(&LayoutParamCallNode { receiver, arguments, block })
}
