// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/string_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamStringNode {
    pub escaped: Document,
}

pub fn layout_string_node(param: &LayoutParamStringNode) -> Document {
    Document::None
}
