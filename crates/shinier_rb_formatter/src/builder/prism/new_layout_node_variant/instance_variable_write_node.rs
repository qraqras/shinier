// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/instance_variable_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamInstanceVariableWriteNode {
    pub name: Document,
    pub value: Document,
}

pub fn layout_instance_variable_write_node(param: &LayoutParamInstanceVariableWriteNode) -> Document {
    Document::None
}
