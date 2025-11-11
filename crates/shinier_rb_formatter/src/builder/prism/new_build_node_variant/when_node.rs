// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/when_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::when_node::{layout_when_node, LayoutParamWhenNode};

fn build_when_node(node: &WhenNode<'_>, context: &mut BuildContext) -> Document {
    let mut conditions = Vec::new();
    for node in &node.conditions() {
        conditions.push(build_node(&node, context));
    }
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_when_node(&LayoutParamWhenNode { conditions, statements })
}
