// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/constant_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::constant_write_node::{layout_constant_write_node, LayoutParamConstantWriteNode};

pub fn build_constant_write_node(node: &ConstantWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_constant_write_node(&LayoutParamConstantWriteNode { value })
}
