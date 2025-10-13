use crate::builder::Buildable;
use crate::doc::{Doc, line, sequence, text, text_from_u8};
use ruby_prism::ConstantWriteNode;

const ASSIGNMENT_OPERATOR: &str = " =";

pub fn build_node(node: Option<&ConstantWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = text_from_u8(node.name().as_slice());
    let operator = text(ASSIGNMENT_OPERATOR);
    let value = node.value().build();
    sequence(&[name, operator, line(), value])
}
