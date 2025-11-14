// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/class_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamClassNode {
    pub superclass: Option<Document>,
    pub body: Option<Document>,
}

pub fn layout_class_node(param: &LayoutParamClassNode) -> Document {
    Document::None
}
