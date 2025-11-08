// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/call_and_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamCallAndWriteNode {
    pub receiver: Option<Document>,
    pub value: Document,
}

pub fn layout_call_and_write_node(param: &LayoutParamCallAndWriteNode) -> Document {
    Document::None
}
