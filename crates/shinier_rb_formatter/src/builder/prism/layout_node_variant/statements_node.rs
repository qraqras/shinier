// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/statements_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamStatementsNode {
    pub body: Vec<Document>,
}

pub fn layout_statements_node(param: &LayoutParamStatementsNode) -> Document {
    Document::None
}
