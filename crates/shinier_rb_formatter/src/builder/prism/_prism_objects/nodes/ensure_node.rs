use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, string};
use crate::document::Document;
use crate::keyword::ENSURE;
use ruby_prism::EnsureNode;

impl<'sh> Build for EnsureNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &EnsureNode, context: &mut BuildContext) -> Document {
    let statements = node.statements();
    group(array(&[
        string(ENSURE),
        indent(array(&[statements.build_with(
            context,
            Some(hardline()),
            None,
        )])),
    ]))
}
