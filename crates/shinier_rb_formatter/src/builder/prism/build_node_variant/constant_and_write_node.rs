// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/constant_and_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::constant_and_write_node::{layout_constant_and_write_node, LayoutParamConstantAndWriteNode};

pub fn build_constant_and_write_node(node: &ConstantAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_constant_and_write_node(&LayoutParamConstantAndWriteNode { value })
}
