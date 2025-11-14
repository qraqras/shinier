// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/super_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamSuperNode {
    pub arguments: Option<Document>,
    pub block: Option<Document>,
}

pub fn layout_super_node(param: &LayoutParamSuperNode) -> Document {
    Document::None
}
