// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/hash_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamHashNode {
    pub elements: Vec<Document>,
}

pub fn layout_hash_node(param: &LayoutParamHashNode) -> Document {
    Document::None
}
