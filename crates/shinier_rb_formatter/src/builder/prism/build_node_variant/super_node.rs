// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/super_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::super_node::{layout_super_node, LayoutParamSuperNode};

pub fn build_super_node(node: &SuperNode<'_>, context: &mut BuildContext) -> Document {
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match &node.block() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_super_node(&LayoutParamSuperNode { arguments, block })
}
