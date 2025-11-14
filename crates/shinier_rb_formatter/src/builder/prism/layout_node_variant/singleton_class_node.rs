// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/singleton_class_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamSingletonClassNode {
    pub expression: Document,
    pub body: Option<Document>,
}

pub fn layout_singleton_class_node(param: &LayoutParamSingletonClassNode) -> Document {
    Document::None
}
