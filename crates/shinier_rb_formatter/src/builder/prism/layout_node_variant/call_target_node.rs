// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/call_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamCallTargetNode {
    pub receiver: Document,
}

pub fn layout_call_target_node(param: &LayoutParamCallTargetNode) -> Document {
    Document::None
}
