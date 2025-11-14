// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/class_variable_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamClassVariableTargetNode {
    pub name: Document,
}

pub fn layout_class_variable_target_node(param: &LayoutParamClassVariableTargetNode) -> Document {
    Document::None
}
