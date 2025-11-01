use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use crate::keyword::{CARET, PARENTHESES};
use ruby_prism::PinnedExpressionNode;

impl<'sh> Build for PinnedExpressionNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &PinnedExpressionNode, context: &mut BuildContext) -> Document {
    let expression = node.expression();
    group(array(&[
        string(CARET),
        string(PARENTHESES.0),
        indent(array(&[softline(), expression.build(context)])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
