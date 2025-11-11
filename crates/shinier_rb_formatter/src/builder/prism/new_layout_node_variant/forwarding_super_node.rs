// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/forwarding_super_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamForwardingSuperNode {
    pub block: Option<Document>,
}

pub fn layout_forwarding_super_node(param: &LayoutParamForwardingSuperNode) -> Document {
    Document::None
}
