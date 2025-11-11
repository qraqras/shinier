// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/assoc_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamAssocNode {
    pub key: Document,
    pub value: Document,
}

pub fn layout_assoc_node(param: &LayoutParamAssocNode) -> Document {
    Document::None
}
