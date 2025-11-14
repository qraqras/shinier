// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/class_variable_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::class_variable_or_write_node::{layout_class_variable_or_write_node, LayoutParamClassVariableOrWriteNode};

pub fn build_class_variable_or_write_node(node: &ClassVariableOrWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_class_variable_or_write_node(&LayoutParamClassVariableOrWriteNode { value })
}
