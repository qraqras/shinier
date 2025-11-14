// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/implicit_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::implicit_node::{layout_implicit_node, LayoutParamImplicitNode};

pub fn build_implicit_node(node: &ImplicitNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_implicit_node(&LayoutParamImplicitNode { value })
}
