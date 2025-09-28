use crate::builder::node::statements_node;
use crate::doc::Doc;
use ruby_prism::ProgramNode;

pub fn print(node: &ProgramNode) -> Doc {
    statements_node::print(&node.statements())
}
