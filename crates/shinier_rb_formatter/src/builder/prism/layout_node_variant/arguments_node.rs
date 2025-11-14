// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/arguments_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamArgumentsNode {
    pub arguments: Vec<Document>,
}

pub fn layout_arguments_node(param: &LayoutParamArgumentsNode) -> Document {
    Document::None
}
