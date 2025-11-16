// filepath: /workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant/hash_node.rs

use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use crate::builder::prism::build_node::build_node;


pub fn build_hash_node(node: &HashNode<'_>, context: &mut BuildContext) -> Document {
    let mut elements = Vec::new();
    for node in &node.elements() {
        elements.push(build_node(&node, context));
    }
    Document::None
}
