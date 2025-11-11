// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/break_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamBreakNode {
    pub arguments: Option<Document>,
}

pub fn layout_break_node(param: &LayoutParamBreakNode) -> Document {
    Document::None
}
