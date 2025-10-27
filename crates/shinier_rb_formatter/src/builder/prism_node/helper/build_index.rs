use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, none, softline, string};
use crate::document::Document;
use crate::keyword::BRACKETS;
use ruby_prism::{Comments, Node};
use std::iter::Peekable;

pub fn build_index(
    receiver_node: Option<&Node>,
    arguments: &[Document],
    comments: &mut Peekable<Comments>,
) -> Document {
    match receiver_node {
        Some(receiver) => group(array(&[
            receiver.build(comments),
            string(BRACKETS.0),
            indent(array(&[softline(), array(arguments)])),
            softline(),
            string(BRACKETS.1),
        ])),
        None => none(),
    }
}
