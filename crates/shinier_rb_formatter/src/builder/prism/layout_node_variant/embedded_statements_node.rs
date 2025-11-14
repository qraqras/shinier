// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/embedded_statements_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamEmbeddedStatementsNode {
    pub statements: Option<Document>,
}

pub fn layout_embedded_statements_node(param: &LayoutParamEmbeddedStatementsNode) -> Document {
    Document::None
}
