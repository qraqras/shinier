use crate::doc::{Doc, text_from_u8};
use ruby_prism::LocalVariableTargetNode;

pub fn build_node(node: &LocalVariableTargetNode) -> Doc {
    let name = node.name();
    text_from_u8(name.as_slice())
}
