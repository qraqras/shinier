// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/local_variable_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamLocalVariableOrWriteNode {
    pub value: Document,
}

pub fn layout_local_variable_or_write_node(param: &LayoutParamLocalVariableOrWriteNode) -> Document {
    Document::None
}
