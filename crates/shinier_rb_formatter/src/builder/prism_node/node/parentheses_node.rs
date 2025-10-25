use crate::builder::Buildable;
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use crate::keyword::PARENTHESES;
use ruby_prism::ParenthesesNode;

pub fn build_node(node: Option<&ParenthesesNode>) -> Document {
    let node = node.unwrap();
    let body = node.body();
    group(array(&[
        string(PARENTHESES.0),
        indent(array(&[softline(), body.build()])),
        softline(),
        string(PARENTHESES.1),
    ]))
}
