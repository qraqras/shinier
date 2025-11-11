// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/symbol_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamSymbolNode {
    pub escaped: Document,
}

pub fn layout_symbol_node(param: &LayoutParamSymbolNode) -> Document {
    Document::None
}
