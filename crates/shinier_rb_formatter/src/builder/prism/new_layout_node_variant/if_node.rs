// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/if_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamIfNode {
    pub statements: Option<Document>,
    pub subsequent: Option<Document>,
}

pub fn layout_if_node(param: &LayoutParamIfNode) -> Document {
    Document::None
}
