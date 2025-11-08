// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/case_match_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamCaseMatchNode {
    pub predicate: Option<Document>,
    pub conditions: Vec<Document>,
    pub else_clause: Option<Document>,
}

pub fn layout_case_match_node(param: &LayoutParamCaseMatchNode) -> Document {
    Document::None
}
