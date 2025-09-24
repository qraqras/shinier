use crate::ast_to_doc::printer;
use crate::doc::*;
use ruby_prism::*;

pub fn print(node: &LocalVariableWriteNode) -> Doc {
    let name = text_u8(node.name().as_slice());
    let operator = text(" =");
    let value = printer::print(&node.value());
    sequence(vec![name, operator, line(), value])
}
