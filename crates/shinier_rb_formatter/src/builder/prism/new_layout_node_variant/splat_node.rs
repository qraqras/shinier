// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/splat_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamSplatNode {
    pub expression: Option<Document>,
}

pub fn layout_splat_node(param: &LayoutParamSplatNode) -> Document {
    Document::None
}
