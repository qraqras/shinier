// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/nil_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::nil_node::{layout_nil_node, LayoutParamNilNode};

fn build_nil_node(node: &NilNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(NIL.to_string());
    layout_nil_node(&LayoutParamNilNode { keyword })
}
