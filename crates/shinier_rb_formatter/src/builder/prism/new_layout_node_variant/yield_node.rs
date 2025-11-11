// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/yield_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamYieldNode {
    pub arguments: Option<Document>,
}

pub fn layout_yield_node(param: &LayoutParamYieldNode) -> Document {
    Document::None
}
