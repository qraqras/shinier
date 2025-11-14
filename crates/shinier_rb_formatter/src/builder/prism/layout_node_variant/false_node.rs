// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/false_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamFalseNode {
    pub keyword: Document,
}

pub fn layout_false_node(param: &LayoutParamFalseNode) -> Document {
    Document::None
}
