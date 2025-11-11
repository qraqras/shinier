// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/integer_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamIntegerNode {
    pub value: Document,
}

pub fn layout_integer_node(param: &LayoutParamIntegerNode) -> Document {
    Document::None
}
