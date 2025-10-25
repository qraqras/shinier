use crate::buildable::Buildable;
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use crate::keyword::{CARET, PARENTHESES};
use ruby_prism::PinnedExpressionNode;

pub fn build_node(node: Option<&PinnedExpressionNode>) -> Document {
    let node = node.unwrap();
    let expression = node.expression();
    group(array(&[
        string(CARET),
        string(PARENTHESES.0),
        indent(array(&[softline(), expression.build()])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
