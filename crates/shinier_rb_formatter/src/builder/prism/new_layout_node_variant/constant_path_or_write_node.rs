// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/constant_path_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamConstantPathOrWriteNode {
    pub target: Document,
    pub value: Document,
}

pub fn layout_constant_path_or_write_node(param: &LayoutParamConstantPathOrWriteNode) -> Document {
    Document::None
}
