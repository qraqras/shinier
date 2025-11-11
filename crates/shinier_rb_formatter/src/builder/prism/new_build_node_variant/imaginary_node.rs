// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/imaginary_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::imaginary_node::{layout_imaginary_node, LayoutParamImaginaryNode};

fn build_imaginary_node(node: &ImaginaryNode<'_>, context: &mut BuildContext) -> Document {
    let numeric = build_node(&node.numeric(), context);
    layout_imaginary_node(&LayoutParamImaginaryNode { numeric })
}
