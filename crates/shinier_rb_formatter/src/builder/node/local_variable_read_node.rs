use crate::doc::*;
use ruby_prism::*;

pub fn build_node(node: &LocalVariableReadNode) -> Doc {
    text_from_u8(node.name().as_slice())
}
