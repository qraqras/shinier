// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/global_variable_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamGlobalVariableWriteNode {
    pub name: Document,
    pub value: Document,
}

pub fn layout_global_variable_write_node(param: &LayoutParamGlobalVariableWriteNode) -> Document {
    Document::None
}
