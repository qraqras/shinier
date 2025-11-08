// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/class_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::class_node::{layout_class_node, LayoutParamClassNode};

fn build_class_node(node: &ClassNode<'_>, context: &mut BuildContext) -> Document {
    build_node(&node.constant_path(), context);
    let superclass = match &node.superclass() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_class_node(&LayoutParamClassNode { superclass, body })
}
