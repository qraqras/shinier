// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/begin_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use crate::keyword::*;
use ruby_prism::*;

pub fn build_begin_node(node: &BeginNode<'_>, context: &mut BuildContext) -> Document {
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let rescue_clause = match &node.rescue_clause() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let else_clause = match &node.else_clause() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let ensure_clause = match &node.ensure_clause() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    Document::None
}
