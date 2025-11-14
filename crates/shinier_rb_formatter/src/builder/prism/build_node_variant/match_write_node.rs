// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/match_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::match_write_node::{
    LayoutParamMatchWriteNode, layout_match_write_node,
};
use crate::keyword::*;
use ruby_prism::*;

pub fn build_match_write_node(node: &MatchWriteNode<'_>, context: &mut BuildContext) -> Document {
    let call = build_node(&node.call().as_node(), context);
    let mut targets = Vec::new();
    for node in &node.targets() {
        targets.push(build_node(&node, context));
    }
    layout_match_write_node(&LayoutParamMatchWriteNode { call, targets })
}
