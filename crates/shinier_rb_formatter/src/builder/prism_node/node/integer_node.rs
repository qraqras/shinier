use crate::builder::Buildable;
use crate::document::*;
use crate::builder::builder::*;
use ruby_prism::IntegerNode;

pub fn build_node(node: Option<&IntegerNode>) -> Doc {
    match node {
        Some(node) => node.value().build(),
        None => none(),
    }
}
