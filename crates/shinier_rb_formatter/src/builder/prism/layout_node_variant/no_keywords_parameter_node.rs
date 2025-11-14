// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/no_keywords_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamNoKeywordsParameterNode {
    pub keyword: Document,
}

pub fn layout_no_keywords_parameter_node(param: &LayoutParamNoKeywordsParameterNode) -> Document {
    Document::None
}
