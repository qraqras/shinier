// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/block_local_variable_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::_new_build_node::build_constant_id;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::BuildContext;
use crate::builder::prism::new_layout_node_variant::block_local_variable_node::{
    LayoutParamBlockLocalVariableNode, layout_block_local_variable_node,
};
use crate::keyword::*;
use ruby_prism::*;

fn build_block_local_variable_node(
    node: &BlockLocalVariableNode<'_>,
    context: &mut BuildContext,
) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_block_local_variable_node(&LayoutParamBlockLocalVariableNode { name })
}
