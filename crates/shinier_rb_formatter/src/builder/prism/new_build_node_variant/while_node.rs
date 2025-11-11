// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/while_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::_new_build_node::build_node;
use crate::builder::prism::new_layout_node_variant::while_node::{layout_while_node, LayoutParamWhileNode};

fn build_while_node(node: &WhileNode<'_>, context: &mut BuildContext) -> Document {
    let predicate = build_node(&node.predicate(), context);
    let statements = match node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_while_node(&LayoutParamWhileNode { predicate, statements })
}
