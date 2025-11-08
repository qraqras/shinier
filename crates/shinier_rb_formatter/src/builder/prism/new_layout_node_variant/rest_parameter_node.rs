// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/rest_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamRestParameterNode {
    pub name: Option<Document>,
}

pub fn layout_rest_parameter_node(param: &LayoutParamRestParameterNode) -> Document {
    Document::None
}
