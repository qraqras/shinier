// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/unless_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamUnlessNode {
    pub predicate: Document,
    pub statements: Option<Document>,
    pub else_clause: Option<Document>,
}

pub fn layout_unless_node(param: &LayoutParamUnlessNode) -> Document {
    Document::None
}
