// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/local_variable_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamLocalVariableTargetNode {
    pub name: Document,
}

pub fn layout_local_variable_target_node(param: &LayoutParamLocalVariableTargetNode) -> Document {
    Document::None
}
