// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/pinned_variable_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamPinnedVariableNode {
    pub variable: Document,
}

pub fn layout_pinned_variable_node(param: &LayoutParamPinnedVariableNode) -> Document {
    Document::None
}
