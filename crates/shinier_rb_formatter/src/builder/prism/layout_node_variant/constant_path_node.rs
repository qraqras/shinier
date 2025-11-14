// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/constant_path_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamConstantPathNode {
    pub parent: Option<Document>,
}

pub fn layout_constant_path_node(param: &LayoutParamConstantPathNode) -> Document {
    Document::None
}
