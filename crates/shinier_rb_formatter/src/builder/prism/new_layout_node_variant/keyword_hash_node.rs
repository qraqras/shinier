// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/keyword_hash_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamKeywordHashNode {
    pub elements: Vec<Document>,
}

pub fn layout_keyword_hash_node(param: &LayoutParamKeywordHashNode) -> Document {
    Document::None
}
