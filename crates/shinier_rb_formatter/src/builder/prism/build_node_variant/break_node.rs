// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/break_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::break_node::{layout_break_node, LayoutParamBreakNode};

pub fn build_break_node(node: &BreakNode<'_>, context: &mut BuildContext) -> Document {
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_break_node(&LayoutParamBreakNode { arguments })
}
