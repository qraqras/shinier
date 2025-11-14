// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/local_variable_read_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamLocalVariableReadNode {
    pub name: Document,
}

pub fn layout_local_variable_read_node(param: &LayoutParamLocalVariableReadNode) -> Document {
    Document::None
}
