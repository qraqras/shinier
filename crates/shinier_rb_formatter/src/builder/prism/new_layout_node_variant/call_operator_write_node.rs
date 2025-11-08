// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/call_operator_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamCallOperatorWriteNode {
    pub receiver: Option<Document>,
    pub value: Document,
}

pub fn layout_call_operator_write_node(param: &LayoutParamCallOperatorWriteNode) -> Document {
    Document::None
}
