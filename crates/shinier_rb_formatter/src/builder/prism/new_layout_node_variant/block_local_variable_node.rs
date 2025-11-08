// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/block_local_variable_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamBlockLocalVariableNode {
    pub name: Document,
}

pub fn layout_block_local_variable_node(param: &LayoutParamBlockLocalVariableNode) -> Document {
    Document::None
}
