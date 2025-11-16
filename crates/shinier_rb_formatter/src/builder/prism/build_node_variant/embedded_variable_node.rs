// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/embedded_variable_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_embedded_variable_node(node: &EmbeddedVariableNode<'_>, context: &mut BuildContext) -> Document {
    let variable = build_node(&node.variable(), context);
    Document::None
}
