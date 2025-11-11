// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/forwarding_arguments_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::forwarding_arguments_node::{layout_forwarding_arguments_node, LayoutParamForwardingArgumentsNode};

fn build_forwarding_arguments_node(node: &ForwardingArgumentsNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(TRIPLE_DOT.to_string());
    layout_forwarding_arguments_node(&LayoutParamForwardingArgumentsNode { keyword })
}
