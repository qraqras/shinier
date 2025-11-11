// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/false_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::false_node::{layout_false_node, LayoutParamFalseNode};

fn build_false_node(node: &FalseNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(FALSE.to_string());
    layout_false_node(&LayoutParamFalseNode { keyword })
}
