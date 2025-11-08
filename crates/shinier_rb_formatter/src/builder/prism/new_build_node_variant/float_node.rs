// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/float_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::_new_build_node::build_f64;
use crate::builder::prism::new_layout_node_variant::float_node::{layout_float_node, LayoutParamFloatNode};

fn build_float_node(node: &FloatNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_f64(&node.value(), context);
    layout_float_node(&LayoutParamFloatNode { value })
}
