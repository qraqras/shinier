// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/self_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::self_node::{layout_self_node, LayoutParamSelfNode};

pub fn build_self_node(node: &SelfNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(SELF.to_string());
    layout_self_node(&LayoutParamSelfNode { keyword })
}
