// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_layout_node_variant/rescue_modifier_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::keyword::*;

pub struct LayoutParamRescueModifierNode {
    pub expression: Document,
    pub rescue_expression: Document,
}

pub fn layout_rescue_modifier_node(param: &LayoutParamRescueModifierNode) -> Document {
    Document::None
}
