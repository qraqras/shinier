use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, string};
use crate::document::Document;
use crate::keyword::ELSE;
use ruby_prism::ElseNode;

impl<'sh> Build for ElseNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ElseNode, context: &mut BuildContext) -> Document {
    let statements = node.statements();
    group(array(&[
        string(ELSE),
        indent(array(&[statements.build_with(
            context,
            Some(hardline()),
            None,
        )])),
    ]))
}
