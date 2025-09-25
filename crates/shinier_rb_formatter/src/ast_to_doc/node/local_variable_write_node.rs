use crate::ast_to_doc::printer;
use crate::consts::ASSIGNMENT_OPERATOR;
use crate::doc::*;
use ruby_prism::*;

pub fn print(node: &LocalVariableWriteNode) -> Doc {
    let name = text_from_u8(node.name().as_slice());
    let operator = text(ASSIGNMENT_OPERATOR);
    let value = printer::print(&node.value());
    sequence(vec![name, operator, line(), value])
}
