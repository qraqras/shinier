use crate::Build;
use crate::BuildContext;
use crate::document::Document;
use ruby_prism::ProgramNode;

impl<'sh> Build for Option<&ProgramNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ProgramNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    statements.as_node().build(context)
}
