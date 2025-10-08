use crate::doc::*;
use ruby_prism::*;

pub fn build_node(node: Option<&LocalVariableReadNode>) -> Doc {
    let node = node.unwrap();
    text_from_u8(node.name().as_slice())
}
