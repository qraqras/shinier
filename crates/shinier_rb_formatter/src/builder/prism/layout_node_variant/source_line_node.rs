// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/source_line_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamSourceLineNode {
    pub keyword: Document,
}

pub fn layout_source_line_node(param: &LayoutParamSourceLineNode) -> Document {
    Document::None
}
