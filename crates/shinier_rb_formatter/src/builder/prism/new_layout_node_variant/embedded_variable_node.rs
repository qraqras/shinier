// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/embedded_variable_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamEmbeddedVariableNode {
    pub variable: Document,
}

pub fn layout_embedded_variable_node(param: &LayoutParamEmbeddedVariableNode) -> Document {
    Document::None
}
