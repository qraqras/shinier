// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/pinned_expression_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamPinnedExpressionNode {
    pub expression: Document,
}

pub fn layout_pinned_expression_node(param: &LayoutParamPinnedExpressionNode) -> Document {
    Document::None
}
