// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/interpolated_string_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamInterpolatedStringNode {
    pub parts: Vec<Document>,
}

pub fn layout_interpolated_string_node(param: &LayoutParamInterpolatedStringNode) -> Document {
    Document::None
}
