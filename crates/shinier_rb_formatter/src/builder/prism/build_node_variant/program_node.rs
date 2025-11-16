use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ProgramNode;

pub fn build_program_node(node: &ProgramNode<'_>, context: &mut BuildContext) -> Document {
    let statements = node.statements();
    array(&[build_node(&statements.as_node(), context), hardline()])
}
