// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/block_parameter_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamBlockParameterNode {
    pub name: Option<Document>,
}

pub fn layout_block_parameter_node(param: &LayoutParamBlockParameterNode) -> Document {
    Document::None
}
