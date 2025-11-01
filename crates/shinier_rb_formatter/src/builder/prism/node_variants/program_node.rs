use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, hardline};
use crate::document::Document;
use ruby_prism::ProgramNode;

impl<'sh> Build for ProgramNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ProgramNode, context: &mut BuildContext) -> Document {
    let statements = node.statements();
    array(&[statements.build(context), hardline()])
}
