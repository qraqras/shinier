// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/hash_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamHashPatternNode {
    pub constant: Option<Document>,
    pub elements: Vec<Document>,
    pub rest: Option<Document>,
}

pub fn layout_hash_pattern_node(param: &LayoutParamHashPatternNode) -> Document {
    Document::None
}
