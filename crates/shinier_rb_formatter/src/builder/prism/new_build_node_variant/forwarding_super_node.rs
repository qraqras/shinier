// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/forwarding_super_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::forwarding_super_node::{layout_forwarding_super_node, LayoutParamForwardingSuperNode};

fn build_forwarding_super_node(node: &ForwardingSuperNode<'_>, context: &mut BuildContext) -> Document {
    let block = match node.block() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_forwarding_super_node(&LayoutParamForwardingSuperNode { block })
}
