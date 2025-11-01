use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, indent, softline, space, string};
use crate::document::Document;
use crate::keyword::{PARENTHESES, SUPER};
use ruby_prism::SuperNode;

impl<'sh> Build for SuperNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &SuperNode, context: &mut BuildContext) -> Document {
    let arguments = node.arguments();
    let block = node.block();
    array(&[
        string(SUPER),
        string(PARENTHESES.0),
        indent(array(&[softline(), arguments.build(context)])),
        softline(),
        string(PARENTHESES.1),
        block.build_with(context, Some(space()), None),
    ])
}
