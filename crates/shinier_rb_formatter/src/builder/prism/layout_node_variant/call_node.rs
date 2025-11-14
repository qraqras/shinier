// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/call_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamCallNode {
    pub receiver: Option<Document>,
    pub arguments: Option<Document>,
    pub block: Option<Document>,
}

pub fn layout_call_node(param: &LayoutParamCallNode) -> Document {
    Document::None
}
