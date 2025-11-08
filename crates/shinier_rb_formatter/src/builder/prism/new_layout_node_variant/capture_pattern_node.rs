// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/capture_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamCapturePatternNode {
    pub value: Document,
    pub target: Document,
}

pub fn layout_capture_pattern_node(param: &LayoutParamCapturePatternNode) -> Document {
    Document::None
}
