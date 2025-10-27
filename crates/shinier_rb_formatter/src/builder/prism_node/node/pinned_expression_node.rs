use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use crate::keyword::{CARET, PARENTHESES};
use ruby_prism::PinnedExpressionNode;

pub fn build_node(node: Option<&PinnedExpressionNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let expression = node.expression();
    group(array(&[
        string(CARET),
        string(PARENTHESES.0),
        indent(array(&[softline(), expression.build(context)])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
