// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/call_or_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamCallOrWriteNode {
    pub receiver: Option<Document>,
    pub value: Document,
}

pub fn layout_call_or_write_node(param: &LayoutParamCallOrWriteNode) -> Document {
    Document::None
}
