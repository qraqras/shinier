// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/next_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamNextNode {
    pub arguments: Option<Document>,
}

pub fn layout_next_node(param: &LayoutParamNextNode) -> Document {
    Document::None
}
