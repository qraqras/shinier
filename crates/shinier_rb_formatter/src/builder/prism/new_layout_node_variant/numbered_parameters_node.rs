// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/numbered_parameters_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamNumberedParametersNode {
    pub maximum: Document,
}

pub fn layout_numbered_parameters_node(param: &LayoutParamNumberedParametersNode) -> Document {
    Document::None
}
