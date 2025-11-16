// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/singleton_class_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_singleton_class_node(node: &SingletonClassNode<'_>, context: &mut BuildContext) -> Document {
    let expression = build_node(&node.expression(), context);
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    Document::None
}
