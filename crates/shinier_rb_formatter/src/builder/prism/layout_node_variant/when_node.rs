// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/when_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamWhenNode {
    pub conditions: Vec<Document>,
    pub statements: Option<Document>,
}

pub fn layout_when_node(param: &LayoutParamWhenNode) -> Document {
    Document::None
}
