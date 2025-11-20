// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/block_parameters_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_block_parameters_node(node: &BlockParametersNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let parameters = match &node.parameters() {
        Some(node) => Some(build_node(&node.as_node(), ctx)),
        None => None,
    };
    let mut locals = Vec::new();
    for node in &node.locals() {
        locals.push(build_node(&node, ctx));
    }
    None
}
