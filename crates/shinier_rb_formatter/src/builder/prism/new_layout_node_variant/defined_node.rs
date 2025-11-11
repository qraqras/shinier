// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/defined_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamDefinedNode {
    pub value: Document,
}

pub fn layout_defined_node(param: &LayoutParamDefinedNode) -> Document {
    Document::None
}
