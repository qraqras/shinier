// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/return_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamReturnNode {
    pub arguments: Option<Document>,
}

pub fn layout_return_node(param: &LayoutParamReturnNode) -> Document {
    Document::None
}
