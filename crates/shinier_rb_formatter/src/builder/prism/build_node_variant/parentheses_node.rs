// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/parentheses_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_parentheses_node(node: &ParenthesesNode<'_>, context: &mut BuildContext) -> Document {
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    Document::None
}
