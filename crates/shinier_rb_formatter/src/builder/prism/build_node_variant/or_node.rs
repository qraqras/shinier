// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/or_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_or_node(node: &OrNode<'_>, context: &mut BuildContext) -> Document {
    let left = build_node(&node.left(), context);
    let right = build_node(&node.right(), context);
    Document::None
}
