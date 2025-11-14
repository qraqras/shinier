// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/instance_variable_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamInstanceVariableTargetNode {
    pub name: Document,
}

pub fn layout_instance_variable_target_node(param: &LayoutParamInstanceVariableTargetNode) -> Document {
    Document::None
}
