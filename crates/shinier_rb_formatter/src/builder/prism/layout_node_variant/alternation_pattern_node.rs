// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/alternation_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamAlternationPatternNode {
    pub left: Document,
    pub right: Document,
}

pub fn layout_alternation_pattern_node(param: &LayoutParamAlternationPatternNode) -> Document {
    Document::None
}
