// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/required_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamRequiredParameterNode {
    pub name: Document,
}

pub fn layout_required_parameter_node(param: &LayoutParamRequiredParameterNode) -> Document {
    Document::None
}
