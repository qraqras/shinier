// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/in_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamInNode {
    pub pattern: Document,
    pub statements: Option<Document>,
}

pub fn layout_in_node(param: &LayoutParamInNode) -> Document {
    Document::None
}
