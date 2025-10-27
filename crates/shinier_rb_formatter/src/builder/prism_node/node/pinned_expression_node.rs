use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use crate::keyword::{CARET, PARENTHESES};
use ruby_prism::Comments;
use ruby_prism::PinnedExpressionNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&PinnedExpressionNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let expression = node.expression();
    group(array(&[
        string(CARET),
        string(PARENTHESES.0),
        indent(array(&[softline(), expression.build(comments)])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
