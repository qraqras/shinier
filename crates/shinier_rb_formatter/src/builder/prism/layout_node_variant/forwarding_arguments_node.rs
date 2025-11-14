// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/forwarding_arguments_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamForwardingArgumentsNode {
    pub keyword: Document,
}

pub fn layout_forwarding_arguments_node(param: &LayoutParamForwardingArgumentsNode) -> Document {
    Document::None
}
