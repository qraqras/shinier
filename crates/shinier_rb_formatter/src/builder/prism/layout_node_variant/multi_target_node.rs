// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/multi_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamMultiTargetNode {
    pub lefts: Vec<Document>,
    pub rest: Option<Document>,
    pub rights: Vec<Document>,
}

pub fn layout_multi_target_node(param: &LayoutParamMultiTargetNode) -> Document {
    Document::None
}
