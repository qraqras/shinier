// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/instance_variable_and_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamInstanceVariableAndWriteNode {
    pub value: Document,
}

pub fn layout_instance_variable_and_write_node(param: &LayoutParamInstanceVariableAndWriteNode) -> Document {
    Document::None
}
