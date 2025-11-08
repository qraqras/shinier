// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/back_reference_read_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamBackReferenceReadNode {
    pub name: Document,
}

pub fn layout_back_reference_read_node(param: &LayoutParamBackReferenceReadNode) -> Document {
    Document::None
}
