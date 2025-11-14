// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/splat_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::splat_node::{layout_splat_node, LayoutParamSplatNode};

pub fn build_splat_node(node: &SplatNode<'_>, context: &mut BuildContext) -> Document {
    let expression = match &node.expression() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_splat_node(&LayoutParamSplatNode { expression })
}
