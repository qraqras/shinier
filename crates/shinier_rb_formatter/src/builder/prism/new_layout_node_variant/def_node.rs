// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/def_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamDefNode {
    pub receiver: Option<Document>,
    pub parameters: Option<Document>,
    pub body: Option<Document>,
}

pub fn layout_def_node(param: &LayoutParamDefNode) -> Document {
    Document::None
}
