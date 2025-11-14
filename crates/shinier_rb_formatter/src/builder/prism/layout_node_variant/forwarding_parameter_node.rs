// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/forwarding_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamForwardingParameterNode {
    pub keyword: Document,
}

pub fn layout_forwarding_parameter_node(param: &LayoutParamForwardingParameterNode) -> Document {
    Document::None
}
