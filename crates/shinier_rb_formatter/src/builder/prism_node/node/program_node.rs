use crate::BuildContext;
use crate::BuildPrismNode;
use crate::document::Document;
use ruby_prism::ProgramNode;

pub fn build_node(node: Option<&ProgramNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    statements.as_node().build(context)
}
