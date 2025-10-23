use crate::buildable::Buildable;
use crate::builder::builder::{array, group, indent, none, softline, string};
use crate::document::Document;
use crate::keyword::BRACKETS;
use ruby_prism::Node;

pub fn build_index(receiver_node: Option<&Node>, arguments: &[Document]) -> Document {
    match receiver_node {
        Some(receiver) => group(array(&[
            receiver.build(),
            string(BRACKETS.0),
            indent(array(&[softline(), array(arguments)])),
            softline(),
            string(BRACKETS.1),
        ])),
        None => none(),
    }
}
