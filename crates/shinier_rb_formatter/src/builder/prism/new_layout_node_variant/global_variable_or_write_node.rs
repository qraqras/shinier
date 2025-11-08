// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/global_variable_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamGlobalVariableOrWriteNode {
    pub value: Document,
}

pub fn layout_global_variable_or_write_node(param: &LayoutParamGlobalVariableOrWriteNode) -> Document {
    Document::None
}
