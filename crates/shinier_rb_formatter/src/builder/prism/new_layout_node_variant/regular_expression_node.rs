// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/regular_expression_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamRegularExpressionNode {
    pub escaped: Document,
}

pub fn layout_regular_expression_node(param: &LayoutParamRegularExpressionNode) -> Document {
    Document::None
}
