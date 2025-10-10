use crate::doc::{Doc, text};
use ruby_prism::IntegerNode;

pub fn build_node(node: Option<&IntegerNode>) -> Doc {
    let node = node.unwrap();
    let value: i32 = node.value().try_into().unwrap();
    return text(value.to_string());
}
