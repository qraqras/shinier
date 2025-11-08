// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/numbered_reference_read_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamNumberedReferenceReadNode {
    pub number: Document,
}

pub fn layout_numbered_reference_read_node(param: &LayoutParamNumberedReferenceReadNode) -> Document {
    Document::None
}
