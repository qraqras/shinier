// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/if_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::if_node::{layout_if_node, LayoutParamIfNode};

pub fn build_if_node(node: &IfNode<'_>, context: &mut BuildContext) -> Document {
    build_node(&node.predicate(), context);
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let subsequent = match &node.subsequent() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_if_node(&LayoutParamIfNode { statements, subsequent })
}
