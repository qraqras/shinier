// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/layout_node_variant/missing_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamMissingNode {
    // Empty
}

pub fn layout_missing_node(param: &LayoutParamMissingNode) -> Document {
    Document::None
}
