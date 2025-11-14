// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/flip_flop_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamFlipFlopNode {
    pub left: Option<Document>,
    pub right: Option<Document>,
}

pub fn layout_flip_flop_node(param: &LayoutParamFlipFlopNode) -> Document {
    Document::None
}
