// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/array_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamArrayPatternNode {
    pub constant: Option<Document>,
    pub requireds: Vec<Document>,
    pub rest: Option<Document>,
    pub posts: Vec<Document>,
}

pub fn layout_array_pattern_node(param: &LayoutParamArrayPatternNode) -> Document {
    Document::None
}
