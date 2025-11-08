// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/constant_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamConstantOrWriteNode {
    pub value: Document,
}

pub fn layout_constant_or_write_node(param: &LayoutParamConstantOrWriteNode) -> Document {
    Document::None
}
