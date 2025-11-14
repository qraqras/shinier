// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/rescue_modifier_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::rescue_modifier_node::{layout_rescue_modifier_node, LayoutParamRescueModifierNode};

pub fn build_rescue_modifier_node(node: &RescueModifierNode<'_>, context: &mut BuildContext) -> Document {
    let expression = build_node(&node.expression(), context);
    let rescue_expression = build_node(&node.rescue_expression(), context);
    layout_rescue_modifier_node(&LayoutParamRescueModifierNode { expression, rescue_expression })
}
