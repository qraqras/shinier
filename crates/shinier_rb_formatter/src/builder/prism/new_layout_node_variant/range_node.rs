// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/range_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamRangeNode {
    pub left: Option<Document>,
    pub right: Option<Document>,
}

pub fn layout_range_node(param: &LayoutParamRangeNode) -> Document {
    Document::None
}
