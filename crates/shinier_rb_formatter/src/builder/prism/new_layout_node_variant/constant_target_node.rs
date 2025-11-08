// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/constant_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamConstantTargetNode {
    pub name: Document,
}

pub fn layout_constant_target_node(param: &LayoutParamConstantTargetNode) -> Document {
    Document::None
}
