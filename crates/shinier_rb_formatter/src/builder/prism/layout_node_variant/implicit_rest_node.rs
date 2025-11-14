// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/implicit_rest_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamImplicitRestNode {
    pub keyword: Document,
}

pub fn layout_implicit_rest_node(param: &LayoutParamImplicitRestNode) -> Document {
    Document::None
}
