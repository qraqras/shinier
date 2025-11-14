// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/block_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::block_node::{layout_block_node, LayoutParamBlockNode};

pub fn build_block_node(node: &BlockNode<'_>, context: &mut BuildContext) -> Document {
    let parameters = match &node.parameters() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_block_node(&LayoutParamBlockNode { parameters, body })
}
