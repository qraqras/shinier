use crate::doc::{Doc, text_from_u8};
use ruby_prism::GlobalVariableReadNode;

pub fn build_node(node: &GlobalVariableReadNode) -> Doc {
    text_from_u8(node.name().as_slice())
}
