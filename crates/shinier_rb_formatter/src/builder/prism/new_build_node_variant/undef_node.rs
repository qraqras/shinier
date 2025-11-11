// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/undef_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::undef_node::{layout_undef_node, LayoutParamUndefNode};

fn build_undef_node(node: &UndefNode<'_>, context: &mut BuildContext) -> Document {
    let mut names = Vec::new();
    for node in &node.names() {
        names.push(build_node(&node, context));
    }
    layout_undef_node(&LayoutParamUndefNode { names })
}
