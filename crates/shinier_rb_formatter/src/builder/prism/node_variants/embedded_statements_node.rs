use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use crate::keyword::{BRACES, HASH};
use ruby_prism::EmbeddedStatementsNode;

impl<'sh> Build for EmbeddedStatementsNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &EmbeddedStatementsNode, context: &mut BuildContext) -> Document {
    let statements = node.statements();
    group(array(&[
        string(HASH),
        string(BRACES.0),
        indent(array(&[softline(), statements.build(context)])),
        softline(),
        string(BRACES.1),
    ]))
}
