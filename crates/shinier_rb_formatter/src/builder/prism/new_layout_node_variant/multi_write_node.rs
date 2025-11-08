// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/multi_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamMultiWriteNode {
    pub lefts: Vec<Document>,
    pub rest: Option<Document>,
    pub rights: Vec<Document>,
    pub value: Document,
}

pub fn layout_multi_write_node(param: &LayoutParamMultiWriteNode) -> Document {
    Document::None
}
