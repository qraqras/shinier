// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/or_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamOrNode {
    pub left: Document,
    pub right: Document,
}

pub fn layout_or_node(param: &LayoutParamOrNode) -> Document {
    Document::None
}
