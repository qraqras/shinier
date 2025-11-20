// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/lambda_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_lambda_node(node: &LambdaNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let parameters = match &node.parameters() {
        Some(node) => Some(build_node(&node, ctx)),
        None => None,
    };
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, ctx)),
        None => None,
    };
    None
}
