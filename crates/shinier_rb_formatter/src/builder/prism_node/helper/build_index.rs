use crate::buildable::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::{BRACKETS, SAFE_NAVIGATION_OPERATOR};
use ruby_prism::Node;

pub fn build_index(
    receiver_node: Option<&Node>,
    arguments: &[Document],
    is_safe_navigation: bool,
) -> Document {
    match receiver_node {
        Some(node) => group(array(&[
            node.build(),
            string(BRACKETS.0),
            softline(),
            match is_safe_navigation {
                true => string(SAFE_NAVIGATION_OPERATOR),
                false => none(),
            },
            indent(array(arguments)),
            softline(),
            string(BRACKETS.1),
        ])),
        None => none(),
    }
}
