// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/shareable_constant_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::shareable_constant_node::{layout_shareable_constant_node, LayoutParamShareableConstantNode};

fn build_shareable_constant_node(node: &ShareableConstantNode<'_>, context: &mut BuildContext) -> Document {
    let write = build_node(&node.write(), context);
    layout_shareable_constant_node(&LayoutParamShareableConstantNode { write })
}
