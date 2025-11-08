// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/rescue_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamRescueNode {
    pub expressions: Vec<Document>,
    pub reference: Option<Document>,
    pub statements: Option<Document>,
    pub subsequent: Option<Document>,
}

pub fn layout_rescue_node(param: &LayoutParamRescueNode) -> Document {
    Document::None
}
