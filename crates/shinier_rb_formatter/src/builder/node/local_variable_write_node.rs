use crate::builder::builder;
use crate::doc::*;
use ruby_prism::*;

const ASSIGNMENT_OPERATOR: &str = " =";

pub fn build_node(node: Option<&LocalVariableWriteNode>) -> Doc {
    let node = node.unwrap();
    let name = text_from_u8(node.name().as_slice());
    let operator = text(ASSIGNMENT_OPERATOR);
    let value = builder::build(&node.value());
    sequence(&[name, operator, line(), value])
}
