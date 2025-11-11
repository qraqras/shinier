// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/constant_path_and_write_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::constant_path_and_write_node::{layout_constant_path_and_write_node, LayoutParamConstantPathAndWriteNode};

fn build_constant_path_and_write_node(node: &ConstantPathAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let target = build_node(&node.target().as_node(), context);
    let value = build_node(&node.value(), context);
    layout_constant_path_and_write_node(&LayoutParamConstantPathAndWriteNode { target, value })
}
