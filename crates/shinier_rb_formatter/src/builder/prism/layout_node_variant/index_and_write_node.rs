// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/index_and_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamIndexAndWriteNode {
    pub receiver: Option<Document>,
    pub arguments: Option<Document>,
    pub block: Option<Document>,
    pub value: Document,
}

pub fn layout_index_and_write_node(param: &LayoutParamIndexAndWriteNode) -> Document {
    Document::None
}
