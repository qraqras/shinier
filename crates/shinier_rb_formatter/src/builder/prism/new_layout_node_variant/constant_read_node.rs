// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/constant_read_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamConstantReadNode {
    pub name: Document,
}

pub fn layout_constant_read_node(param: &LayoutParamConstantReadNode) -> Document {
    Document::None
}
