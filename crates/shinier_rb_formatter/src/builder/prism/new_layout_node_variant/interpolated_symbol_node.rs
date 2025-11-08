// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/interpolated_symbol_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamInterpolatedSymbolNode {
    pub parts: Vec<Document>,
}

pub fn layout_interpolated_symbol_node(param: &LayoutParamInterpolatedSymbolNode) -> Document {
    Document::None
}
