// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/constant_path_operator_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamConstantPathOperatorWriteNode {
    pub target: Document,
    pub value: Document,
}

pub fn layout_constant_path_operator_write_node(param: &LayoutParamConstantPathOperatorWriteNode) -> Document {
    Document::None
}
