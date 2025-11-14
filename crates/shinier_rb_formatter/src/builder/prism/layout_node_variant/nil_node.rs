// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/nil_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamNilNode {
    pub keyword: Document,
}

pub fn layout_nil_node(param: &LayoutParamNilNode) -> Document {
    Document::None
}
