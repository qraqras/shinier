// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/begin_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamBeginNode {
    pub statements: Option<Document>,
    pub rescue_clause: Option<Document>,
    pub else_clause: Option<Document>,
    pub ensure_clause: Option<Document>,
}

pub fn layout_begin_node(param: &LayoutParamBeginNode) -> Document {
    Document::None
}
