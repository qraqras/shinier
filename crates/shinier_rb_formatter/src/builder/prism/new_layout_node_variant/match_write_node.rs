// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/match_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamMatchWriteNode {
    pub call: Document,
    pub targets: Vec<Document>,
}

pub fn layout_match_write_node(param: &LayoutParamMatchWriteNode) -> Document {
    Document::None
}
