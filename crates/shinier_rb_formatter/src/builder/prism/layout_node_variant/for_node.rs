// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/for_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamForNode {
    pub index: Document,
    pub collection: Document,
    pub statements: Option<Document>,
}

pub fn layout_for_node(param: &LayoutParamForNode) -> Document {
    Document::None
}
