// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/local_variable_and_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamLocalVariableAndWriteNode {
    pub value: Document,
}

pub fn layout_local_variable_and_write_node(param: &LayoutParamLocalVariableAndWriteNode) -> Document {
    Document::None
}
