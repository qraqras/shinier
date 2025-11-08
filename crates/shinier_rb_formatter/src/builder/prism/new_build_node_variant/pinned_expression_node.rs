// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/pinned_expression_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::pinned_expression_node::{layout_pinned_expression_node, LayoutParamPinnedExpressionNode};

fn build_pinned_expression_node(node: &PinnedExpressionNode<'_>, context: &mut BuildContext) -> Document {
    let expression = build_node(&node.expression(), context);
    layout_pinned_expression_node(&LayoutParamPinnedExpressionNode { expression })
}
