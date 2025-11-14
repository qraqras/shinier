// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/implicit_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamImplicitNode {
    pub value: Document,
}

pub fn layout_implicit_node(param: &LayoutParamImplicitNode) -> Document {
    Document::None
}
