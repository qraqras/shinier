use crate::doc::{Doc, text};
use ruby_prism::IntegerNode;

pub fn build_node(node: &IntegerNode) -> Doc {
    let value: i32 = node.value().try_into().unwrap();
    return text(value.to_string());
}
