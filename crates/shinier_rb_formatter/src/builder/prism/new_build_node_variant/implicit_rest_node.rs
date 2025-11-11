// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/implicit_rest_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::implicit_rest_node::{layout_implicit_rest_node, LayoutParamImplicitRestNode};

fn build_implicit_rest_node(node: &ImplicitRestNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(COMMA.to_string());
    layout_implicit_rest_node(&LayoutParamImplicitRestNode { keyword })
}
