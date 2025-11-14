// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/rational_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::layout_node_variant::rational_node::{layout_rational_node, LayoutParamRationalNode};

pub fn build_rational_node(node: &RationalNode<'_>, context: &mut BuildContext) -> Document {
    // TODO
    layout_rational_node(&LayoutParamRationalNode {  })
}
