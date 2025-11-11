// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/imaginary_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamImaginaryNode {
    pub numeric: Document,
}

pub fn layout_imaginary_node(param: &LayoutParamImaginaryNode) -> Document {
    Document::None
}
