use crate::builder::node::statements_node;
use crate::doc::Doc;
use ruby_prism::ProgramNode;

pub fn build_node(node: &ProgramNode) -> Doc {
    statements_node::build_node(&node.statements())
}
