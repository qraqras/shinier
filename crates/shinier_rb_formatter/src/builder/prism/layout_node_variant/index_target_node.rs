// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/index_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamIndexTargetNode {
    pub arguments: Option<Document>,
    pub block: Option<Document>,
}

pub fn layout_index_target_node(param: &LayoutParamIndexTargetNode) -> Document {
    Document::None
}
