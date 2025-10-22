use crate::builder::Buildable;
use crate::document::*;
use ruby_prism::ProgramNode;

pub fn build_node(node: Option<&ProgramNode>) -> Doc {
    let node = node.unwrap();
    let statements = node.statements();
    statements.as_node().build()
}
