use crate::doc::{Doc, text_from_u8};
use ruby_prism::ConstantReadNode;

pub fn build_node(node: Option<&ConstantReadNode>) -> Doc {
    let node = node.unwrap();
    text_from_u8(node.name().as_slice())
}
