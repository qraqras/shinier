// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/block_argument_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::block_argument_node::{layout_block_argument_node, LayoutParamBlockArgumentNode};

fn build_block_argument_node(node: &BlockArgumentNode<'_>, context: &mut BuildContext) -> Document {
    let expression = match &node.expression() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_block_argument_node(&LayoutParamBlockArgumentNode { expression })
}
