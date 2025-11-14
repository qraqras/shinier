// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/constant_path_target_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamConstantPathTargetNode {
    pub parent: Option<Document>,
}

pub fn layout_constant_path_target_node(param: &LayoutParamConstantPathTargetNode) -> Document {
    Document::None
}
