// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/class_variable_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamClassVariableOrWriteNode {
    pub value: Document,
}

pub fn layout_class_variable_or_write_node(param: &LayoutParamClassVariableOrWriteNode) -> Document {
    Document::None
}
