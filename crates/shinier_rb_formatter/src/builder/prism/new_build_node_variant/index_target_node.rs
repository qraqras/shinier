// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/index_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::index_target_node::{layout_index_target_node, LayoutParamIndexTargetNode};

fn build_index_target_node(node: &IndexTargetNode<'_>, context: &mut BuildContext) -> Document {
    build_node(&node.receiver(), context);
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match &node.block() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_index_target_node(&LayoutParamIndexTargetNode { arguments, block })
}
