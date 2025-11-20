// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/call_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_call_or_write_node(node: &CallOrWriteNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, ctx)),
        None => None,
    };
    let value = build_node(&node.value(), ctx);
    None
}
