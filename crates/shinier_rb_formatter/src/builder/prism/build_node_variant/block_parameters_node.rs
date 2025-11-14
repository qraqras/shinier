// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/block_parameters_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::block_parameters_node::{layout_block_parameters_node, LayoutParamBlockParametersNode};

pub fn build_block_parameters_node(node: &BlockParametersNode<'_>, context: &mut BuildContext) -> Document {
    let parameters = match &node.parameters() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let mut locals = Vec::new();
    for node in &node.locals() {
        locals.push(build_node(&node, context));
    }
    layout_block_parameters_node(&LayoutParamBlockParametersNode { parameters, locals })
}
