// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/block_parameters_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamBlockParametersNode {
    pub parameters: Option<Document>,
    pub locals: Vec<Document>,
}

pub fn layout_block_parameters_node(param: &LayoutParamBlockParametersNode) -> Document {
    Document::None
}
