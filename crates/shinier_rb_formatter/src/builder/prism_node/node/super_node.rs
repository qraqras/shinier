use crate::buildable::Buildable;
use crate::builder::builder::{array, indent, softline, space, string};
use crate::document::Document;
use crate::keyword::{PARENTHESES, SUPER};
use ruby_prism::SuperNode;

pub fn build_node(node: Option<&SuperNode>) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    let block = node.block();
    array(&[
        string(SUPER),
        string(PARENTHESES.0),
        indent(array(&[softline(), arguments.build()])),
        softline(),
        string(PARENTHESES.1),
        block.build_with(Some(space()), None),
    ])
}
