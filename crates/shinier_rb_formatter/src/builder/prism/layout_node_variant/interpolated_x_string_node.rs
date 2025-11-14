// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/interpolated_x_string_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamInterpolatedXStringNode {
    pub parts: Vec<Document>,
}

pub fn layout_interpolated_x_string_node(param: &LayoutParamInterpolatedXStringNode) -> Document {
    Document::None
}
