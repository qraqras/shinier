// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/self_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamSelfNode {
    pub keyword: Document,
}

pub fn layout_self_node(param: &LayoutParamSelfNode) -> Document {
    Document::None
}
