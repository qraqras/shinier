// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/find_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamFindPatternNode {
    pub constant: Option<Document>,
    pub left: Document,
    pub requireds: Vec<Document>,
    pub right: Document,
}

pub fn layout_find_pattern_node(param: &LayoutParamFindPatternNode) -> Document {
    Document::None
}
