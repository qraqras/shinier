// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/def_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::def_node::{layout_def_node, LayoutParamDefNode};

fn build_def_node(node: &DefNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let parameters = match &node.parameters() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_def_node(&LayoutParamDefNode { receiver, parameters, body })
}
