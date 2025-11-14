// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/constant_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamConstantWriteNode {
    pub value: Document,
}

pub fn layout_constant_write_node(param: &LayoutParamConstantWriteNode) -> Document {
    Document::None
}
