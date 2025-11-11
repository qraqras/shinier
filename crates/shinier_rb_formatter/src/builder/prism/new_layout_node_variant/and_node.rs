// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/and_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamAndNode {
    pub left: Document,
    pub right: Document,
}

pub fn layout_and_node(param: &LayoutParamAndNode) -> Document {
    Document::None
}
