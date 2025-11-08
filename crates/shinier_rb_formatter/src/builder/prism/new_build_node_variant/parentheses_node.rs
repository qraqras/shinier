// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/parentheses_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::parentheses_node::{layout_parentheses_node, LayoutParamParenthesesNode};

fn build_parentheses_node(node: &ParenthesesNode<'_>, context: &mut BuildContext) -> Document {
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_parentheses_node(&LayoutParamParenthesesNode { body })
}
