// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/integer_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::_new_build_node::build_integer;
use crate::builder::prism::new_layout_node_variant::integer_node::{layout_integer_node, LayoutParamIntegerNode};

fn build_integer_node(node: &IntegerNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_integer(&node.value(), context);
    layout_integer_node(&LayoutParamIntegerNode { value })
}
