use crate::builder::Buildable;
use crate::doc::{Doc, line, sequence, text};
use ruby_prism::ConstantWriteNode;

const ASSIGNMENT_OPERATOR: &str = " =";

pub fn build_node(node: Option<&ConstantWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name().build();
    let operator = text(ASSIGNMENT_OPERATOR);
    let value = node.value().build();
    sequence(&[name, operator, line(), value])
}
