// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/and_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::and_node::{layout_and_node, LayoutParamAndNode};

fn build_and_node(node: &AndNode<'_>, context: &mut BuildContext) -> Document {
    let left = build_node(&node.left(), context);
    let right = build_node(&node.right(), context);
    layout_and_node(&LayoutParamAndNode{ left, right })
}
