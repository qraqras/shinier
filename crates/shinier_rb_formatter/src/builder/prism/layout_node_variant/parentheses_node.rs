// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/parentheses_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamParenthesesNode {
    pub body: Option<Document>,
}

pub fn layout_parentheses_node(param: &LayoutParamParenthesesNode) -> Document {
    Document::None
}
