// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/instance_variable_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamInstanceVariableOrWriteNode {
    pub value: Document,
}

pub fn layout_instance_variable_or_write_node(param: &LayoutParamInstanceVariableOrWriteNode) -> Document {
    Document::None
}
