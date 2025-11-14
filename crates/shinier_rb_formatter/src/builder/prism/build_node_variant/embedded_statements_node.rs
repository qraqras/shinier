// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/embedded_statements_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::embedded_statements_node::{layout_embedded_statements_node, LayoutParamEmbeddedStatementsNode};

pub fn build_embedded_statements_node(node: &EmbeddedStatementsNode<'_>, context: &mut BuildContext) -> Document {
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_embedded_statements_node(&LayoutParamEmbeddedStatementsNode { statements })
}
