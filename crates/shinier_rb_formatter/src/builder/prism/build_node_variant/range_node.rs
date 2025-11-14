// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/range_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::range_node::{layout_range_node, LayoutParamRangeNode};

pub fn build_range_node(node: &RangeNode<'_>, context: &mut BuildContext) -> Document {
    let left = match &node.left() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let right = match &node.right() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_range_node(&LayoutParamRangeNode { left, right })
}
