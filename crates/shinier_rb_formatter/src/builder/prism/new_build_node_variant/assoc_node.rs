// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/assoc_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::assoc_node::{layout_assoc_node, LayoutParamAssocNode};

fn build_assoc_node(node: &AssocNode<'_>, context: &mut BuildContext) -> Document {
    let key = build_node(&node.key(), context);
    let value = build_node(&node.value(), context);
    layout_assoc_node(&LayoutParamAssocNode { key, value })
}
