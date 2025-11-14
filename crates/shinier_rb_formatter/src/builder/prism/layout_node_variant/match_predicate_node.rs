// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/match_predicate_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamMatchPredicateNode {
    pub value: Document,
    pub pattern: Document,
}

pub fn layout_match_predicate_node(param: &LayoutParamMatchPredicateNode) -> Document {
    Document::None
}
