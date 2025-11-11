// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/true_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::true_node::{layout_true_node, LayoutParamTrueNode};

fn build_true_node(node: &TrueNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(TRUE.to_string());
    layout_true_node(&LayoutParamTrueNode { keyword })
}
