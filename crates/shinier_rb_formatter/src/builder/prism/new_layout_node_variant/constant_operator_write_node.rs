// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/constant_operator_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamConstantOperatorWriteNode {
    pub value: Document,
}

pub fn layout_constant_operator_write_node(param: &LayoutParamConstantOperatorWriteNode) -> Document {
    Document::None
}
