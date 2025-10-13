use crate::doc::{Doc, text_from_u8};
use ruby_prism::LocalVariableTargetNode;

pub fn build_node(node: Option<&LocalVariableTargetNode>) -> Doc {
    let node = node.unwrap();
    let name = node.name();
    text_from_u8(name.as_slice())
}
