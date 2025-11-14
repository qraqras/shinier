use crate::Document;
use crate::builder::builder::*;

pub struct LayoutParamProgramNode {
    pub statements: Document,
}

pub fn layout_program_node(param: LayoutParamProgramNode) -> Document {
    array(&[param.statements, hardline()])
}
