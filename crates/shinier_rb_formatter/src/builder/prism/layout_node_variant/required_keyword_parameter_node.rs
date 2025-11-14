// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/required_keyword_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamRequiredKeywordParameterNode {
    pub name: Document,
}

pub fn layout_required_keyword_parameter_node(param: &LayoutParamRequiredKeywordParameterNode) -> Document {
    Document::None
}
