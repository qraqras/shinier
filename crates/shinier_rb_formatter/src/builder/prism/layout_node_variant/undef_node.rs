// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/undef_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamUndefNode {
    pub names: Vec<Document>,
}

pub fn layout_undef_node(param: &LayoutParamUndefNode) -> Document {
    Document::None
}
