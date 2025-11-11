// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/true_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamTrueNode {
    pub keyword: Document,
}

pub fn layout_true_node(param: &LayoutParamTrueNode) -> Document {
    Document::None
}
