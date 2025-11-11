// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/class_variable_read_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamClassVariableReadNode {
    pub name: Document,
}

pub fn layout_class_variable_read_node(param: &LayoutParamClassVariableReadNode) -> Document {
    Document::None
}
