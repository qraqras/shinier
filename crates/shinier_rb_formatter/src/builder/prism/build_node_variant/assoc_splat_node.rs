// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/assoc_splat_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::assoc_splat_node::{layout_assoc_splat_node, LayoutParamAssocSplatNode};

pub fn build_assoc_splat_node(node: &AssocSplatNode<'_>, context: &mut BuildContext) -> Document {
    let value = match &node.value() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_assoc_splat_node(&LayoutParamAssocSplatNode { value } )
}
