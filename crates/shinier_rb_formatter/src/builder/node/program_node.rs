use crate::builder::build;
use crate::doc::Doc;
use ruby_prism::ProgramNode;

pub fn build_node(node: Option<&ProgramNode>) -> Doc {
    let node = node.unwrap();
    let statements = node.statements();
    build(&statements.as_node())
}
