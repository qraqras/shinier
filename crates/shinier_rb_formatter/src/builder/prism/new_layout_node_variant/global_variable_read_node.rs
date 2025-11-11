// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/global_variable_read_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamGlobalVariableReadNode {
    pub name: Document,
}

pub fn layout_global_variable_read_node(param: &LayoutParamGlobalVariableReadNode) -> Document {
    Document::None
}
