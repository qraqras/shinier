// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/float_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::build_node::build_f64;


pub fn build_float_node(node: &FloatNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_f64(&node.value(), context);
    Document::None
}
