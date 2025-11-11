// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/statements_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::statements_node::{layout_statements_node, LayoutParamStatementsNode};

fn build_statements_node(node: &StatementsNode<'_>, context: &mut BuildContext) -> Document {
    let mut body = Vec::new();
    for node in &node.body() {
        body.push(build_node(&node, context));
    }
    layout_statements_node(&LayoutParamStatementsNode { body })
}
