// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/assoc_splat_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamAssocSplatNode {
    pub value: Option<Document>,
}

pub fn layout_assoc_splat_node(param: &LayoutParamAssocSplatNode) -> Document {
    Document::None
}
