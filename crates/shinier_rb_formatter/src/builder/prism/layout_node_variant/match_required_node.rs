// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/match_required_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamMatchRequiredNode {
    pub value: Document,
    pub pattern: Document,
}

pub fn layout_match_required_node(param: &LayoutParamMatchRequiredNode) -> Document {
    Document::None
}
