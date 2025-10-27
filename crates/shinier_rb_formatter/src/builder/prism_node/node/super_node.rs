use crate::BuildPrismNode;
use crate::builder::builder::{array, indent, softline, space, string};
use crate::document::Document;
use crate::keyword::{PARENTHESES, SUPER};
use ruby_prism::Comments;
use ruby_prism::SuperNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&SuperNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    let block = node.block();
    array(&[
        string(SUPER),
        string(PARENTHESES.0),
        indent(array(&[softline(), arguments.build(comments)])),
        softline(),
        string(PARENTHESES.1),
        block.build_with(comments, Some(space()), None),
    ])
}
