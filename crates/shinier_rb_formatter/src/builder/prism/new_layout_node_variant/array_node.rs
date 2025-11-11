// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/array_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamArrayNode {
    pub elements: Vec<Document>,
}

pub fn layout_array_node(param: &LayoutParamArrayNode) -> Document {
    Document::None
}
