use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ProgramNode;

pub fn build_program_node(node: &ProgramNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let statements = node.statements();
    build_node(statements.as_node(), ctx)
}
