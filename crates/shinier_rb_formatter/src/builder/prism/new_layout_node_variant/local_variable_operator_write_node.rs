// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/local_variable_operator_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamLocalVariableOperatorWriteNode {
    pub value: Document,
}

pub fn layout_local_variable_operator_write_node(param: &LayoutParamLocalVariableOperatorWriteNode) -> Document {
    Document::None
}
