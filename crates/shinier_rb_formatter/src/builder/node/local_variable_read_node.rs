use crate::doc::*;
use ruby_prism::*;

pub fn print(node: &LocalVariableReadNode) -> Doc {
    text_from_u8(node.name().as_slice())
}
