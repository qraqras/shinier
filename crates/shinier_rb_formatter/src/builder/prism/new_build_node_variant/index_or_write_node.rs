// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/index_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::index_or_write_node::{layout_index_or_write_node, LayoutParamIndexOrWriteNode};

fn build_index_or_write_node(node: &IndexOrWriteNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match &node.block() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let value = build_node(&node.value(), context);
    layout_index_or_write_node(&LayoutParamIndexOrWriteNode { receiver, arguments, block, value })
}
