// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/optional_keyword_param_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamOptionalKeywordParameterNode {
    pub value: Document,
}

pub fn layout_optional_keyword_parameter_node(
    param: &LayoutParamOptionalKeywordParameterNode,
) -> Document {
    Document::None
}
