// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/keyword_rest_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamKeywordRestParameterNode {
    pub name: Option<Document>,
}

pub fn layout_keyword_rest_parameter_node(param: &LayoutParamKeywordRestParameterNode) -> Document {
    Document::None
}
