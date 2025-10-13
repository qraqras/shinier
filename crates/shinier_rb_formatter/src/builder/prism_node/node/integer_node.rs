use crate::builder::Buildable;
use crate::doc::{Doc, none};
use ruby_prism::IntegerNode;

pub fn build_node(node: Option<&IntegerNode>) -> Doc {
    match node {
        Some(node) => node.value().build(),
        None => none(),
    }
}
