// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/for_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::for_node::{layout_for_node, LayoutParamForNode};

pub fn build_for_node(node: &ForNode<'_>, context: &mut BuildContext) -> Document {
    let index = build_node(&node.index(), context);
    let collection = build_node(&node.collection(), context);
    let statements = match node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_for_node(&LayoutParamForNode { index, collection, statements })
}
