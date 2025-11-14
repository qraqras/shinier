// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/index_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamIndexOrWriteNode {
    pub receiver: Option<Document>,
    pub arguments: Option<Document>,
    pub block: Option<Document>,
    pub value: Document,
}

pub fn layout_index_or_write_node(param: &LayoutParamIndexOrWriteNode) -> Document {
    Document::None
}
