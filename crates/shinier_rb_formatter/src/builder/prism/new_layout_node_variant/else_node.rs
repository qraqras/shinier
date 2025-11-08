// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/else_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamElseNode {
    pub statements: Option<Document>,
}

pub fn layout_else_node(param: &LayoutParamElseNode) -> Document {
    Document::None
}
