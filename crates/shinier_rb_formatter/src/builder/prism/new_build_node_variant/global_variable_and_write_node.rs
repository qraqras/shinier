// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/global_variable_and_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::global_variable_and_write_node::{layout_global_variable_and_write_node, LayoutParamGlobalVariableAndWriteNode};

fn build_global_variable_and_write_node(node: &GlobalVariableAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_global_variable_and_write_node(&LayoutParamGlobalVariableAndWriteNode { value })
}
