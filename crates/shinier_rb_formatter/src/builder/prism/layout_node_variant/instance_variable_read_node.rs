// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/instance_variable_read_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamInstanceVariableReadNode {
    pub name: Document,
}

pub fn layout_instance_variable_read_node(param: &LayoutParamInstanceVariableReadNode) -> Document {
    Document::None
}
