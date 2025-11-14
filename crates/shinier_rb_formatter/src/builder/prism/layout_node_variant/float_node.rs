// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/float_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamFloatNode {
    pub value: Document,
}

pub fn layout_float_node(param: &LayoutParamFloatNode) -> Document {
    Document::None
}
