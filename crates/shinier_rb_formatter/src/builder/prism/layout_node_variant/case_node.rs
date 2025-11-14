// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/case_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamCaseNode {
    pub predicate: Option<Document>,
    pub conditions: Vec<Document>,
    pub else_clause: Option<Document>,
}

pub fn layout_case_node(param: &LayoutParamCaseNode) -> Document {
    Document::None
}
