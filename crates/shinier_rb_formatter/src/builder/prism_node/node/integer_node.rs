use crate::builder::Buildable;
use crate::builder::builder::none;
use crate::document::Document;
use ruby_prism::IntegerNode;

pub fn build_node(node: Option<&IntegerNode>) -> Document {
    match node {
        Some(node) => node.value().build(),
        None => none(),
    }
}
