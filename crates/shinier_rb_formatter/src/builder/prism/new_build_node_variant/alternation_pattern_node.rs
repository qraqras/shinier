// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/alternation_pattern_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::alternation_pattern_node::{layout_alternation_pattern_node, LayoutParamAlternationPatternNode};

fn build_alternation_pattern_node(node: &AlternationPatternNode<'_>, context: &mut BuildContext) -> Document {
    let left = build_node(&node.left(), context);
    let right = build_node(&node.right(), context);
    layout_alternation_pattern_node(&LayoutParamAlternationPatternNode { left, right })
}
