// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/pinned_variable_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::pinned_variable_node::{layout_pinned_variable_node, LayoutParamPinnedVariableNode};

fn build_pinned_variable_node(node: &PinnedVariableNode<'_>, context: &mut BuildContext) -> Document {
    let variable = build_node(&node.variable(), context);
    layout_pinned_variable_node(&LayoutParamPinnedVariableNode { variable })
}
