// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/until_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamUntilNode {
    pub predicate: Document,
    pub statements: Option<Document>,
}

pub fn layout_until_node(param: &LayoutParamUntilNode) -> Document {
    Document::None
}
