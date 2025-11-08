// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/constant_path_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::constant_path_node::{layout_constant_path_node, LayoutParamConstantPathNode};

fn build_constant_path_node(node: &ConstantPathNode<'_>, context: &mut BuildContext) -> Document {
    let parent = match &node.parent() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_constant_path_node(&LayoutParamConstantPathNode { parent })
}
