// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/interpolated_regular_expression_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::interpolated_regular_expression_node::{layout_interpolated_regular_expression_node, LayoutParamInterpolatedRegularExpressionNode};

fn build_interpolated_regular_expression_node(node: &InterpolatedRegularExpressionNode<'_>, context: &mut BuildContext) -> Document {
    let mut parts = Vec::new();
    for node in &node.parts() {
        parts.push(build_node(&node, context));
    }
    layout_interpolated_regular_expression_node(&LayoutParamInterpolatedRegularExpressionNode { parts })
}
