// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/while_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamWhileNode {
    pub predicate: Document,
    pub statements: Option<Document>,
}

pub fn layout_while_node(param: &LayoutParamWhileNode) -> Document {
    Document::None
}
