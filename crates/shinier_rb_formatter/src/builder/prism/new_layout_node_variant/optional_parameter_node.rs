// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/optional_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamOptionalParameterNode {
    pub value: Document,
}

pub fn layout_optional_parameter_node(param: &LayoutParamOptionalParameterNode) -> Document {
    Document::None
}
